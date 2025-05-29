use super::config::{CartPoleConfig, KinematicsIntegrator};
use crate::{
    env::{Environment, Error},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::{SVector, DMatrix, DVector};
use rayon::prelude::*;

const FOUR_THIRDS: f64 = 4.0 / 3.0;
const STATE_SIZE: usize = 4;
const ACTION_SIZE: usize = 1;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;
type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

#[derive(Debug)]
pub struct VectorizedCartPole {
    num_envs: usize,
    t: Vec<u32>,
    config: CartPoleConfig,
    states: Option<DMatrix<f64>>, // Shape: [STATE_SIZE, num_envs]
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl VectorizedCartPole {
    pub fn new(config: CartPoleConfig, num_envs: usize) -> Result<Self, Error> {
        let m = config.mc + config.mp;
        let high: State = SVector::from_vec(vec![
            config.x_max * 2.0,
            (2.0 * config.f * config.x_max / m).sqrt(),
            config.theta_max * 2.0,
            ((2.0 * config.f * config.theta_max) / (config.l * (FOUR_THIRDS - config.mp / m)))
                .sqrt(),
        ]);

        let space = EnvSpace {
            state: Boxed::new(-high, high)?,
            action: match config.continuous {
                false => Mixed::discrete(2)?,
                true => Mixed::continuous(SVector::from_element(-1.0), SVector::from_element(1.0))?,
            },
        };

        Ok(VectorizedCartPole {
            num_envs,
            space,
            config,
            t: vec![0; num_envs],
            states: None,
        })
    }

    // Vectorized acceleration computation using SIMD-friendly operations
    fn compute_accelerations(&self, states: &DMatrix<f64>, forces: &DVector<f64>) -> (DVector<f64>, DVector<f64>) {
        let theta_row = states.row(2);
        let omega_row = states.row(3);
        
        // Use nalgebra's vectorized operations
        let sin_theta = theta_row.map(|x| x.sin());
        let cos_theta = theta_row.map(|x| x.cos());
        
        let m = self.config.mc + self.config.mp;
        let mpl = self.config.mp * self.config.l;
        
        // Vectorized computation
        let omega_squared = omega_row.component_mul(&omega_row);
        let t = (forces + mpl * omega_squared.component_mul(&sin_theta)) / m;
        
        let num = self.config.g * &sin_theta - cos_theta.component_mul(&t);
        let cos_theta_squared = cos_theta.component_mul(&cos_theta);
        let den = self.config.l * (FOUR_THIRDS - self.config.mp * cos_theta_squared / m);
        
        let theta_acc = num.component_div(&den);
        let x_acc = &t - (mpl / m) * theta_acc.component_mul(&cos_theta);
        
        (x_acc.into_owned(), theta_acc.into_owned())
    }

    fn integrate_batch(&self, states: &DMatrix<f64>, forces: &DVector<f64>) -> DMatrix<f64> {
        let (x_acc, theta_acc) = self.compute_accelerations(states, forces);
        
        match self.config.integrator {
            KinematicsIntegrator::Euler => self.euler_batch(states, &x_acc, &theta_acc),
            KinematicsIntegrator::SemiImplicitEuler => {
                self.semi_implicit_euler_batch(states, &x_acc, &theta_acc)
            }
        }
    }

    fn euler_batch(&self, states: &DMatrix<f64>, x_acc: &DVector<f64>, theta_acc: &DVector<f64>) -> DMatrix<f64> {
        let mut new_states = DMatrix::zeros(STATE_SIZE, self.num_envs);
        let tau = self.config.tau;
        
        // Vectorized Euler integration
        new_states.set_row(0, &(states.row(0) + tau * states.row(1)));
        new_states.set_row(1, &(states.row(1) + tau * x_acc.transpose()));
        new_states.set_row(2, &(states.row(2) + tau * states.row(3)));
        new_states.set_row(3, &(states.row(3) + tau * theta_acc.transpose()));
        
        new_states
    }

    fn semi_implicit_euler_batch(&self, states: &DMatrix<f64>, x_acc: &DVector<f64>, theta_acc: &DVector<f64>) -> DMatrix<f64> {
        let mut new_states = DMatrix::zeros(STATE_SIZE, self.num_envs);
        let tau = self.config.tau;
        
        // Update velocities first
        let new_v = states.row(1) + tau * x_acc.transpose();
        let new_omega = states.row(3) + tau * theta_acc.transpose();
        
        // Then update positions with new velocities
        new_states.set_row(0, &(states.row(0) + tau * &new_v));
        new_states.set_row(1, &new_v);
        new_states.set_row(2, &(states.row(2) + tau * &new_omega));
        new_states.set_row(3, &new_omega);
        
        new_states
    }

    fn forces_from_actions(&self, actions: &[Action]) -> Result<DVector<f64>, Error> {
        let mut forces = DVector::zeros(self.num_envs);
        
        for (i, action) in actions.iter().enumerate() {
            forces[i] = match (&self.space.action, action) {
                (Mixed::Discrete(space), Action::Discrete(act)) => {
                    space.contains(act).map_err(|_| Error::InvalidAction)?;
                    (2.0 * *act as f64 - 1.0) * self.config.f
                }
                (Mixed::Continuous(space), Action::Continuous(act)) => {
                    space.contains(act).map_err(|_| Error::InvalidAction)?;
                    act[0] * self.config.f
                }
                _ => return Err(Error::InvalidAction),
            };
        }
        
        Ok(forces)
    }

    pub fn reset_all(&mut self, seed: Option<u64>) -> Result<Vec<State>, Error> {
        let mut states = DMatrix::zeros(STATE_SIZE, self.num_envs);
        
        // Reset each environment with different seeds if provided
        for i in 0..self.num_envs {
            let env_seed = seed.map(|s| s.wrapping_add(i as u64));
            let state = self.space.state.uniform(env_seed, -5e-2, 5e-2)?;
            states.set_column(i, &state);
        }
        
        self.t.fill(0);
        self.states = Some(states.clone());
        
        // Convert back to vector of states
        let result: Vec<State> = (0..self.num_envs)
            .map(|i| State::from_column_slice(states.column(i).as_slice()))
            .collect();
        
        Ok(result)
    }

    pub fn step_all(&mut self, actions: &[Action]) -> Result<(Vec<State>, Vec<f64>, Vec<bool>, Vec<Option<()>>), Error> {
        if actions.len() != self.num_envs {
            return Err(Error::InvalidAction);
        }

        let states = self.states.as_ref().ok_or(Error::NotInitialized)?;
        let forces = self.forces_from_actions(actions)?;
        let new_states = self.integrate_batch(states, &forces);
        
        // Update time steps
        for t in &mut self.t {
            *t += 1;
        }
        
        self.states = Some(new_states.clone());
        
        // Convert results
        let states_vec: Vec<State> = (0..self.num_envs)
            .map(|i| State::from_column_slice(new_states.column(i).as_slice()))
            .collect();
        
        let rewards = vec![1.0; self.num_envs]; // All environments get reward 1.0
        
        let dones: Vec<bool> = (0..self.num_envs)
            .map(|i| {
                let state = new_states.column(i);
                self.t[i] > self.config.t_max
                    || state[0].abs() > self.config.x_max
                    || state[2].abs() > self.config.theta_max
            })
            .collect();
        
        let infos = vec![None; self.num_envs];
        
        Ok((states_vec, rewards, dones, infos))
    }

    // Reset only specific environments that are done
    pub fn reset_done(&mut self, dones: &[bool], seed: Option<u64>) -> Result<(), Error> {
        if let Some(ref mut states) = self.states {
            for (i, &is_done) in dones.iter().enumerate() {
                if is_done {
                    let env_seed = seed.map(|s| s.wrapping_add(i as u64));
                    let new_state = self.space.state.uniform(env_seed, -5e-2, 5e-2)?;
                    states.set_column(i, &new_state);
                    self.t[i] = 0;
                }
            }
        }
        Ok(())
    }
}

// Optional: Parallel processing version using Rayon
impl VectorizedCartPole {
    pub fn step_all_parallel(&mut self, actions: &[Action]) -> Result<(Vec<State>, Vec<f64>, Vec<bool>, Vec<Option<()>>), Error> {
        if actions.len() != self.num_envs {
            return Err(Error::InvalidAction);
        }

        let states = self.states.as_ref().ok_or(Error::NotInitialized)?;
        
        // Process in parallel chunks for very large numbers of environments
        let chunk_size = (self.num_envs / rayon::current_num_threads()).max(1);
        let results: Result<Vec<_>, Error> = (0..self.num_envs)
            .into_par_iter()
            .chunks(chunk_size)
            .map(|chunk| {
                let mut chunk_results = Vec::new();
                for i in chunk {
                    let state = State::from_column_slice(states.column(i).as_slice());
                    let force = match (&self.space.action, &actions[i]) {
                        (Mixed::Discrete(space), Action::Discrete(act)) => {
                            space.contains(act).map_err(|_| Error::InvalidAction)?;
                            (2.0 * *act as f64 - 1.0) * self.config.f
                        }
                        (Mixed::Continuous(space), Action::Continuous(act)) => {
                            space.contains(act).map_err(|_| Error::InvalidAction)?;
                            act[0] * self.config.f
                        }
                        _ => return Err(Error::InvalidAction),
                    };
                    
                    let (x_acc, theta_acc) = self.compute_acceleration_single(&state, &force);
                    let new_state = self.integrate_single(&state, &x_acc, &theta_acc);
                    chunk_results.push((new_state, i));
                }
                Ok(chunk_results)
            })
            .collect();

        // Update states and compute results...
        // (Implementation continues with result aggregation)
        
        todo!("Complete parallel implementation")
    }

    fn compute_acceleration_single(&self, state: &State, force: &f64) -> (f64, f64) {
        // Single environment version for parallel processing
        let theta: f64 = state[2];
        let omega: f64 = state[3];

        let (sin_theta, cos_theta) = theta.sin_cos();

        let m: f64 = self.config.mc + self.config.mp;
        let mpl: f64 = self.config.mp * self.config.l;

        let t: f64 = (force + mpl * omega.powi(2) * sin_theta) / m;

        let num: f64 = self.config.g * sin_theta - cos_theta * t;
        let den: f64 = self.config.l * (FOUR_THIRDS - self.config.mp * cos_theta.powi(2) / m);
        let theta_acc: f64 = num / den;
        let x_acc: f64 = t - mpl * theta_acc * cos_theta / m;

        (x_acc, theta_acc)
    }

    fn integrate_single(&self, state: &State, x_acc: &f64, theta_acc: &f64) -> State {
        match self.config.integrator {
            KinematicsIntegrator::Euler => {
                let x = state[0];
                let v = state[1];
                let theta = state[2];
                let omega = state[3];
                SVector::from_vec(vec![
                    x + self.config.tau * v,
                    v + self.config.tau * x_acc,
                    theta + self.config.tau * omega,
                    omega + self.config.tau * theta_acc,
                ])
            }
            KinematicsIntegrator::SemiImplicitEuler => {
                let x = state[0];
                let v = state[1] + self.config.tau * x_acc;
                let theta = state[2];
                let omega = state[3] + self.config.tau * theta_acc;
                SVector::from_vec(vec![
                    x + self.config.tau * v,
                    v,
                    theta + self.config.tau * omega,
                    omega,
                ])
            }
        }
    }
}