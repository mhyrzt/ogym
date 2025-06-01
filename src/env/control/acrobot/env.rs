use std::f64::consts::{PI, TAU};

use nalgebra::SVector;
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

use crate::{
    env::environment::{Environment, Error, Experience, Terminal},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};

use super::{AcrobotConfig, config::DynamicsMode};

const ACTION_SIZE: usize = 1; // Discrete(3) or optionally continuous
const RAW_STATE_SIZE: usize = 4;
const STATE_SIZE: usize = 6;

type RawState = SVector<f64, RAW_STATE_SIZE>;
type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

#[derive(Debug)]
pub struct Acrobot {
    pub config: AcrobotConfig,
    t: u32,
    raw_state: Option<RawState>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl Acrobot {
    pub fn new(config: AcrobotConfig) -> Result<Self, Error> {
        let ha = SVector::from_element(1.0);
        let hs = SVector::from_vec(vec![1., 1., 1., 1., 4. * PI, 9. * PI]);
        let space = EnvSpace {
            state: Boxed::new(-hs, hs)?,
            action: match config.continuous {
                true => Mixed::continuous(-ha, ha)?,
                false => Mixed::discrete(3)?,
            },
        };
        Ok(Self {
            config,
            space,
            t: 0,
            raw_state: None,
        })
    }

    fn raw(&self) -> Result<RawState, Error> {
        match self.raw_state {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }

    fn tau(&self, action: &Action) -> Result<f64, Error> {
        let mut tau = match (&self.space.action, action) {
            (Mixed::Discrete(space), MixedItem::Discrete(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                (*act - 1) as f64
            }
            (Mixed::Continuous(space), MixedItem::Continuous(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                act[0]
            }
            _ => return Err(Error::InvalidAction),
        };

        if self.config.torque_noise_max >= 0.0 {
            tau += StdRng::from_os_rng()
                .random_range(-self.config.torque_noise_max..self.config.torque_noise_max);
        }
        Ok(tau)
    }

    fn ds_dt(&self, state: RawState, tau: f64) -> SVector<f64, RAW_STATE_SIZE> {
        let g = self.config.g;
        let m1 = self.config.link_mass_1;
        let m2 = self.config.link_mass_2;
        let l1 = self.config.link_length_1;
        let c1 = self.config.link_com_pos_1;
        let c2 = self.config.link_com_pos_2;
        let moi = self.config.link_moi;
        let th1 = state[0];
        let th2 = state[1];
        let dt1 = state[2];
        let dt2 = state[3];

        let (sin_th2, cos_th2) = th2.sin_cos();

        let d1 = m1 * c1 * c1 + m2 * (l1 * l1 + c2 * c2 + 2.0 * l1 * c2 * cos_th2) + moi * 2.0;
        let d2 = m2 * (c2 * c2 + l1 * c2 * cos_th2) + moi;

        let phi2 = m2 * c2 * g * (th1 + th2 - PI / 2.0).cos();
        let phi1 = -m2 * l1 * c2 * dt2 * dt2 * sin_th2 - 2.0 * m2 * l1 * c2 * dt2 * dt1 * sin_th2
            + (m1 * c1 + m2 * l1) * g * (th1 - PI / 2.0).cos()
            + phi2;

        let alpha2 = match self.config.dynamics_mode {
            DynamicsMode::Nips => {
                // Consistent with the paper
                (tau + d2 / d1 * phi1 - phi2) / (m2 * c2 * c2 + moi - d2 * d2 / d1)
            }
            DynamicsMode::Book => {
                // Consistent with java implementation and book
                (tau + d2 / d1 * phi1 - m2 * l1 * c2 * dt1 * dt1 * sin_th2 - phi2)
                    / (m2 * c2 * c2 + moi - d2 * d2 / d1)
            }
        };
        let alpha1 = -(d2 * alpha2 + phi1) / d1;

        SVector::<f64, RAW_STATE_SIZE>::new(dt1, dt2, alpha1, alpha2)
    }

    fn constraint(&self, mut state: RawState) -> RawState {
        state[0] = (state[0] + PI).rem_euclid(TAU) - PI;
        state[1] = (state[1] + PI).rem_euclid(TAU) - PI;
        state[2] = state[2].clamp(-self.config.max_vel_1, self.config.max_vel_1);
        state[3] = state[3].clamp(-self.config.max_vel_2, self.config.max_vel_2);

        state
    }

    fn rk4(&self, state: RawState, tau: f64) -> RawState {
        let dt = self.config.dt;
        let dt2 = dt / 2.0;

        let k1 = self.ds_dt(state, tau);
        let k2 = self.ds_dt(state + dt2 * k1, tau);
        let k3 = self.ds_dt(state + dt2 * k2, tau);
        let k4 = self.ds_dt(state + dt * k3, tau);

        self.constraint(state + dt / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4))
    }
}

impl Environment for Acrobot {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.t = 0;
        let mut rng = match seed {
            Some(state) => StdRng::seed_from_u64(state),
            None => StdRng::from_rng(&mut rand::rng()),
        };
        let dist = Uniform::new(-0.1, 0.1)?;
        let raw_state: SVector<f64, RAW_STATE_SIZE> =
            SVector::from_fn(|_, _| dist.sample(&mut rng));

        self.raw_state = Some(raw_state);
        Ok((self.state()?, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let curr_state = self.state()?;

        self.raw_state = Some(self.rk4(self.raw()?, self.tau(&action)?));
        let next_state = self.state()?;

        self.t += 1;

        Ok(Experience::new(
            curr_state,
            if self.is_terminal()? { 0.0 } else { -1.0 },
            action,
            next_state,
            (),
            self.to_terminal()?,
            self.t,
        ))
    }

    fn state(&self) -> Result<Self::State, Error> {
        let s = self.raw()?;
        let (sin_s0, cos_s0) = s[0].sin_cos();
        let (sin_s1, cos_s1) = s[1].sin_cos();

        Ok(SVector::from_vec(vec![
            cos_s0, sin_s0, cos_s1, sin_s1, s[2], s[3],
        ]))
    }

    #[inline]
    fn is_terminal(&self) -> Result<bool, Error> {
        let s = self.raw()?;
        Ok(s[0].cos() + (s[0] + s[1]).cos() < -1.)
    }

    #[inline]
    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_t
    }
}
