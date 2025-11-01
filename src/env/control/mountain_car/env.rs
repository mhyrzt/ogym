use super::{MountainCarConfig, config::MountainCarReward};
use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::SVector;

pub const ACTION_SIZE: usize = 1;
pub const STATE_SIZE: usize = 2;

type Action = MixedItem<ACTION_SIZE>;
pub type ActionSpace = Mixed<ACTION_SIZE>;

type State = SVector<f64, STATE_SIZE>;
pub type StateSpace = Boxed<STATE_SIZE>;

pub struct MountainCar {
    t: u32,
    state: Option<State>,
    config: MountainCarConfig,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl MountainCar {
    pub fn new(config: MountainCarConfig) -> Result<Self, Error> {
        let state = Boxed::new(
            SVector::from_vec(vec![config.min_x, -config.max_v]),
            SVector::from_vec(vec![config.max_x, config.max_v]),
        )?;

        let action = match config.continuous {
            true => Mixed::continuous(SVector::from_element(-1.0), SVector::from_element(1.0))?,
            false => Mixed::discrete(3)?,
        };

        Ok(Self {
            t: 0,
            state: None,
            config,
            space: EnvSpace { state, action },
        })
    }

    fn reward(&self, action: &Action) -> f64 {
        -1.0 - match self.config.reward {
            MountainCarReward::Constant => 0.0,
            MountainCarReward::ActionPenalty => match action {
                MixedItem::Discrete(_) => 0.0,
                MixedItem::Continuous(a) => a.norm_squared(),
            },
        }
    }

    fn force(&self, action: &Action) -> f64 {
        match action {
            MixedItem::Discrete(a) => *a as f64 - 1.0,
            MixedItem::Continuous(a) => a[0],
        }
    }

    fn clamp_velocity_at_boundary(&self, x: &f64, v: &f64) -> f64 {
        if *x <= self.config.min_x && *v < 0.0 {
            0.0
        } else {
            *v
        }
    }
}

impl Environment for MountainCar {
    type Action = Action;
    type Info = Option<()>;
    type State = State;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        let mut state = self.space.state.uniform(seed, -0.6, -0.4)?;
        state[1] = 0.0;
        self.t = 0;
        self.state = Some(state);
        Ok((state, None))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }
        self.space.action.contains(&action)?;
        let curr_state = self.state()?;
        let v: f64 = (curr_state[1] + self.force(&action) * self.config.f
            - 25e-4 * (3.0 * curr_state[0]).cos())
        .clamp(-self.config.max_v, self.config.max_v);
        let x = (curr_state[0] + v).clamp(self.config.min_x, self.config.max_x);

        let next_state = SVector::from_vec(vec![x, self.clamp_velocity_at_boundary(&x, &v)]);
        self.state = Some(next_state);
        self.t += 1;

        Ok(Experience::new(
            curr_state,
            self.reward(&action),
            action,
            next_state,
            None,
            self.to_terminal()?,
            self.t,
        ))
    }

    fn is_done(&self) -> Result<bool, Error> {
        let state = self.state()?;
        let x = state[0];
        let v = state[1];
        Ok(self.t > self.config.max_t || (x >= self.config.goal_x && v >= self.config.goal_v))
    }

    fn state(&self) -> Result<Self::State, Error> {
        match self.state {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        let state = self.state()?;
        let x = state[0];
        let v = state[1];
        Ok(x >= self.config.goal_x && v >= self.config.goal_v)
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_t
    }
}
