use std::f64::consts::{PI, TAU};

use super::PendulumConfig;
use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::SVector;
use rand::{Rng, SeedableRng, rngs::StdRng};

const STATE_SIZE: usize = 3;
const ACTION_SIZE: usize = 1;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

fn normalize_angle(x: f64) -> f64 {
    (x + PI).rem_euclid(TAU) - PI
}

#[derive(Debug)]
pub struct Pendulum {
    t: u32,
    config: PendulumConfig,
    state: Option<State>,
    theta: f64,
    omega: f64,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl Pendulum {
    pub fn new(config: PendulumConfig) -> Result<Self, Error> {
        let hs = SVector::from_vec(vec![1.0, 1.0, 8.0]);
        let ha = SVector::from_element(1.0);

        Ok(Self {
            space: EnvSpace {
                state: Boxed::new(-hs, hs)?,
                action: match config.continuous {
                    true => Mixed::discrete(config.n)?,
                    false => Mixed::continuous(-ha, ha)?,
                },
            },
            config,
            t: 0,
            state: None,
            theta: 0.,
            omega: 0.,
        })
    }

    fn rand_theta_omega(&self, seed: Option<u64>) -> (f64, f64) {
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        let theta = rng.random_range(-self.config.x0..self.config.x0);
        let omega = rng.random_range(-self.config.y0..self.config.y0);

        (theta, omega)
    }

    fn to_state(&self) -> SVector<f64, STATE_SIZE> {
        let (ts, tc) = self.theta.sin_cos();
        SVector::from_vec(vec![ts, tc, self.omega])
    }
}

impl Environment for Pendulum {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.t = 0;
        (self.theta, self.omega) = self.rand_theta_omega(seed);
        let s = self.to_state();
        self.state = Some(s);
        Ok((s, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.state()?;
        self.space.action.contains(&action)?;
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let u = match action {
            MixedItem::Discrete(a) => {
                (a as f64) * (2.0 * self.config.max_tau) / (self.config.n as f64 - 1.)
                    - self.config.max_tau
            }
            MixedItem::Continuous(a) => a[0] * self.config.max_tau,
        };
        let g = self.config.g;
        let l = self.config.l;
        let m = self.config.m;
        let dt = self.config.dt;

        let cost =
            normalize_angle(self.theta).powi(2) + 0.1 * self.omega.powi(2) + 1e-3 * u.powi(2);

        self.omega = (self.omega
            + dt * (3. * g * self.theta.sin() / (2. * l) + 3. * u / (m * l.powi(2))))
        .clamp(-self.config.max_tau, self.config.max_tau);

        self.theta += self.omega * dt;

        let curr_state = self.state()?;
        let next_state = self.to_state();
        self.state = Some(next_state);
        self.t += 1;

        Ok(Experience::new(
            curr_state,
            -cost,
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
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_t
    }
}
