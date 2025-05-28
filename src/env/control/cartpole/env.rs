use super::config::{CartPoleConfig, KinematicsIntegrator};
use super::error::CartPoleError;
use crate::{
    env::Environment,
    spaces::{
        continuous::{ContinuousSpace, ContinuousSpaceError},
        discrete::{DiscreteSpace, DiscreteSpaceError},
        space::{EnvSpace, Space},
    },
};
use nalgebra::DVector;
use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

const FOUR_THIRDS: f64 = 4.0 / 3.0;

pub struct CartPole {
    t: u32,
    config: CartPoleConfig,
    state: Option<DVector<f64>>,
    pub space: EnvSpace<ContinuousSpace, DiscreteSpace>,
}

impl CartPole {
    pub fn new() -> Result<Self, CartPoleError> {
        let config = CartPoleConfig::default();

        let high: DVector<f64> = DVector::from_vec(vec![
            config.x_max * 2.0,
            f64::INFINITY,
            config.theta_max * 2.0,
            f64::INFINITY,
        ]);

        let space = EnvSpace {
            state: ContinuousSpace::new(-high.clone(), high)?,
            action: DiscreteSpace::new(2)?,
        };
        Ok(CartPole {
            space,
            config,
            t: 0,
            state: None,
        })
    }

    fn compute_acceleration(&self, state: &DVector<f64>, force: &f64) -> (f64, f64) {
        let theta: f64 = state[2];
        let omega: f64 = state[3];

        let sin_theta: f64 = theta.sin();
        let cos_theta: f64 = theta.cos();
        let omega_sq: f64 = omega.powi(2);

        let m: f64 = self.config.mc + self.config.mp;
        let mpl: f64 = self.config.mp * self.config.l;

        let t: f64 = (force + mpl * omega_sq * sin_theta) / m;

        let num: f64 = self.config.g * sin_theta - cos_theta * t;
        let den: f64 = self.config.l * (FOUR_THIRDS - self.config.mp * cos_theta.powi(2) / m);
        let theta_acc: f64 = num / den;
        let x_acc: f64 = t - mpl * theta_acc * cos_theta / m;

        (x_acc, theta_acc)
    }

    fn euler(&self, state: &DVector<f64>, x_acc: &f64, theta_acc: &f64) -> DVector<f64> {
        let x = state[0];
        let v = state[1];
        let theta = state[2];
        let omega = state[3];
        DVector::from_vec(vec![
            x + self.config.tau * v,
            v + self.config.tau * x_acc,
            theta + self.config.tau * omega,
            omega + self.config.tau * theta_acc,
        ])
    }

    fn semi_implicit_euler(
        &self,
        state: &DVector<f64>,
        x_acc: &f64,
        theta_acc: &f64,
    ) -> DVector<f64> {
        let x = state[0];
        let v = state[1] + self.config.tau * x_acc;
        let theta = state[2];
        let omega = state[3] + self.config.tau * theta_acc;
        DVector::from_vec(vec![
            x + self.config.tau * v,
            v,
            theta + self.config.tau * omega,
            omega,
        ])
    }

    fn integrate(&self, state: &DVector<f64>, force: &f64) -> DVector<f64> {
        let (x_acc, theta_acc) = self.compute_acceleration(state, force);
        match self.config.integrator {
            KinematicsIntegrator::Euler => self.euler(state, &x_acc, &theta_acc),
            KinematicsIntegrator::SemiImplicitEuler => {
                self.semi_implicit_euler(state, &x_acc, &theta_acc)
            }
        }
    }
}

impl Environment for CartPole {
    type Action = u32;
    type State = DVector<f64>;
    type Info = Option<String>;
    type Error = CartPoleError;
    type ActionError = DiscreteSpaceError;
    type StateError = ContinuousSpaceError;
    type ActionSpace = DiscreteSpace;
    type StateSpace = ContinuousSpace;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Self::Error> {
        self.t = 0;
        let mut rng = match seed {
            Some(state) => StdRng::seed_from_u64(state),
            None => StdRng::from_rng(&mut rand::rng()),
        };
        let size = self.space.state.low.len();
        let range = Uniform::new(-0.05, 0.05)?;
        let data: Vec<f64> = (0..size).map(|_| range.sample(&mut rng)).collect();
        let state = DVector::from_vec(data);
        self.state = Some(state.clone());
        Ok((state, None))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<(Self::State, f64, bool, Self::Info), Self::Error> {
        self.space.action.contains(&action)?;
        if self.is_done()? {
            return Err(CartPoleError::EpisodeDone);
        }
        let state = self.state()?;
        let force: f64 = (2.0 * action as f64 - 1.0) * self.config.f;
        self.t += 1;
        let new_state = self.integrate(&state, &force);
        self.state = Some(new_state.clone());

        Ok((new_state, 1.0, self.is_done()?, None))
    }

    fn is_done(&self) -> Result<bool, Self::Error> {
        let state = self.state()?;
        Ok(self.t > self.config.t_max
            || state[0].abs() > self.config.x_max
            || state[2].abs() > self.config.theta_max)
    }

    fn state(&self) -> Result<Self::State, Self::Error> {
        match &self.state {
            Some(s) => Ok(s.clone()),
            None => Err(CartPoleError::NotInitialized),
        }
    }
}
