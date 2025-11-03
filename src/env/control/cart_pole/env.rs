use super::config::{CartPoleConfig, KinematicsIntegrator};
use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::SVector;

const FOUR_THIRDS: f64 = 4.0 / 3.0;
const STATE_SIZE: usize = 4;
const ACTION_SIZE: usize = 1;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

#[derive(Debug)]
pub struct CartPole {
    t: u32,
    config: CartPoleConfig,
    state: Option<State>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl CartPole {
    pub fn new(config: CartPoleConfig) -> Result<Self, Error> {
        // The formulas for v_max and omega_max are derived based on energy conservation principles.
        // For a detailed explanation and derivation, see: https://chatgpt.com/share/68387c8f-2298-800e-bfd1-b9af501d3b30
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
        Ok(CartPole {
            space,
            config,
            t: 0,
            state: None,
        })
    }

    fn compute_acceleration(&self, state: &State, force: &f64) -> (f64, f64) {
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

    fn euler(&self, state: &State, x_acc: &f64, theta_acc: &f64) -> State {
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

    fn semi_implicit_euler(&self, state: &State, x_acc: &f64, theta_acc: &f64) -> State {
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

    fn integrate(&self, state: &State, force: &f64) -> State {
        let (x_acc, theta_acc) = self.compute_acceleration(state, force);
        match self.config.integrator {
            KinematicsIntegrator::Euler => self.euler(state, &x_acc, &theta_acc),
            KinematicsIntegrator::SemiImplicitEuler => {
                self.semi_implicit_euler(state, &x_acc, &theta_acc)
            }
        }
    }

    fn force(&self, action: &Action) -> Result<f64, Error> {
        match (&self.space.action, action) {
            (Mixed::Discrete(space), Action::Discrete(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                Ok((2.0 * *act as f64 - 1.0) * self.config.f)
            }
            (Mixed::Continuous(space), Action::Continuous(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                Ok(act[0] * self.config.f)
            }
            _ => Err(Error::InvalidAction),
        }
    }
}

impl Environment for CartPole {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        let state = self.space.state.uniform(seed, -5e-2, 5e-2)?;
        self.t = 0;
        self.state = Some(state);
        Ok((state, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, self::Action>, Error> {
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let curr_state = self.state()?;
        let force: f64 = self.force(&action)?;
        let next_state = self.integrate(&curr_state, &force);
        self.state = Some(next_state);
        self.t += 1;

        Ok(Experience::new(
            curr_state,
            -1.,
            action,
            next_state,
            (),
            self.to_terminal()?,
            self.t,
        ))
    }

    fn state(&self) -> Result<Self::State, Error> {
        match self.state {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        let state = self.state()?;
        Ok(state[0].abs() > self.config.x_max || state[2].abs() > self.config.theta_max)
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.t_max
    }
}
