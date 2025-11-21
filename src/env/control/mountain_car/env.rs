use super::{config::MountainCarReward, MountainCarConfig};
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

    pub fn t(&self) -> u32 {
        self.t
    }

    pub fn reward(&self, action: &Action) -> f64 {
        -1.0 - match self.config.reward {
            MountainCarReward::Constant => 0.0,
            MountainCarReward::ActionPenalty => match action {
                MixedItem::Discrete(_) => 0.0,
                MixedItem::Continuous(a) => a.norm_squared(),
            },
        }
    }

    pub fn force(&self, action: &Action) -> f64 {
        match action {
            MixedItem::Discrete(a) => *a as f64 - 1.0,
            MixedItem::Continuous(a) => a[0],
        }
    }

    pub fn clamp_velocity_at_boundary(&self, x: &f64, v: &f64) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::Space;
    use nalgebra::SVector;

    fn vec2(x: f64, y: f64) -> SVector<f64, 2> {
        SVector::from_vec(vec![x, y])
    }

    fn vec1(x: f64) -> SVector<f64, 1> {
        SVector::from_vec(vec![x])
    }

    #[test]
    fn test_mountain_car_new_discrete() {
        let config = MountainCarConfig::default().with_discrete_action();
        let env = MountainCar::new(config);

        assert!(env.is_ok());

        let env = env.unwrap();
        match env.space.action.sample() {
            Ok(MixedItem::Discrete(_)) => assert!(true),
            _ => panic!("Expected discrete action space"),
        }
    }

    #[test]
    fn test_mountain_car_new_continuous() {
        let config = MountainCarConfig::default().with_continuous_action();
        let env = MountainCar::new(config).unwrap();

        match env.space.action.sample() {
            Ok(MixedItem::Continuous(_)) => assert!(true),
            _ => panic!("Expected continuous action space"),
        }
    }

    #[test]
    fn test_mountain_car_force_discrete() {
        let config = MountainCarConfig::default();
        let env = MountainCar::new(config).unwrap();

        assert_eq!(env.force(&MixedItem::Discrete(0)), -1.0);
        assert_eq!(env.force(&MixedItem::Discrete(1)), 0.0);
        assert_eq!(env.force(&MixedItem::Discrete(2)), 1.0);
    }

    #[test]
    fn test_mountain_car_force_continuous() {
        let config = MountainCarConfig::default().with_continuous_action();
        let env = MountainCar::new(config).unwrap();

        let action = MixedItem::Continuous(vec1(0.5));
        assert_eq!(env.force(&action), 0.5);
    }

    #[test]
    fn test_mountain_car_clamp_velocity_at_boundary() {
        let config = MountainCarConfig::default();
        let env = MountainCar::new(config).unwrap();

        assert_eq!(env.clamp_velocity_at_boundary(&config.min_x, &-0.01), 0.0);
        assert_eq!(
            env.clamp_velocity_at_boundary(&(config.min_x + 0.1), &-0.01),
            -0.01
        );
        assert_eq!(env.clamp_velocity_at_boundary(&config.min_x, &0.01), 0.01);
    }

    #[test]
    fn test_reward_constant() {
        let config = MountainCarConfig::default().with_constant_reward();
        let env = MountainCar::new(config).unwrap();
        let action = MixedItem::Discrete(1);
        assert_eq!(env.reward(&action), -1.0);
    }

    #[test]
    fn test_reward_action_penalty_discrete() {
        let config = MountainCarConfig::default().with_action_penalty_reward();
        let env = MountainCar::new(config).unwrap();

        let action = MixedItem::Discrete(1);
        assert_eq!(env.reward(&action), -1.0);
    }

    #[test]
    fn test_reward_action_penalty_continuous() {
        let config = MountainCarConfig::default()
            .with_action_penalty_reward()
            .with_continuous_action();
        let env = MountainCar::new(config).unwrap();

        let action = MixedItem::Continuous(vec1(0.5));
        assert_eq!(env.reward(&action), -1.25);
    }

    #[test]
    fn test_state_space_bounds() {
        let config = MountainCarConfig::default();
        let env = MountainCar::new(config).unwrap();

        let low = env.space.state.low;
        let high = env.space.state.high;

        assert_eq!(low[0], config.min_x);
        assert_eq!(low[1], -config.max_v);
        assert_eq!(high[0], config.max_x);
        assert_eq!(high[1], config.max_v);
    }

    #[test]
    fn test_step_logic_discrete() {
        let config = MountainCarConfig::default().with_discrete_action();
        let mut env = MountainCar::new(config).unwrap();

        let (initial_state, _) = env.reset(None).unwrap();
        let action = MixedItem::Discrete(2);
        let experience = env.step(action).unwrap();

        assert_eq!(experience.curr_state, initial_state);
        assert_ne!(experience.next_state, initial_state);
        assert_eq!(experience.step, 1);
    }

    #[test]
    fn test_invalid_action() {
        let config = MountainCarConfig::default().with_discrete_action();
        let mut env = MountainCar::new(config).unwrap();
        env.reset(None).unwrap();

        let invalid_action = MixedItem::Discrete(5);
        assert!(env.step(invalid_action).is_err());
    }

    #[test]
    fn test_reset_functionality() {
        let config = MountainCarConfig::default();
        let mut env = MountainCar::new(config).unwrap();

        let (state, _) = env.reset(None).unwrap();

        assert_eq!(state[1], 0.0);
        assert!(state[0] >= -0.6 && state[0] <= -0.4);
        assert_eq!(env.t(), 0);
    }

    #[test]
    fn test_termination_conditions() {
        let config = MountainCarConfig::default().with_max_steps(5);
        let mut env = MountainCar::new(config).unwrap();
        env.reset(None).unwrap();

        for i in 1..=5 {
            let action = MixedItem::Discrete(1);
            let exp = env.step(action).unwrap();

            if i < 5 {
                assert!(!exp.terminal.is_done());
            } else {
                assert!(exp.terminal.is_done());
                assert!(exp.terminal.is_truncated());
            }
        }

        assert!(env.is_done().unwrap());
    }

    #[test]
    fn test_goal_termination() {
        let config = MountainCarConfig::default();
        let mut env = MountainCar::new(config).unwrap();
        env.reset(None).unwrap();

        let goal_state = vec2(0.6, 0.01);
        env.state = Some(goal_state);

        assert!(env.is_terminal().unwrap());
        assert!(env.is_done().unwrap());
    }

    #[test]
    fn test_episode_done_error() {
        let config = MountainCarConfig::default().with_max_steps(1);
        let mut env = MountainCar::new(config).unwrap();
        env.reset(None).unwrap();

        let action = MixedItem::Discrete(0);
        let _ = env.step(action).unwrap();

        let result = env.step(action);
        match result {
            Err(Error::EpisodeDone) => assert!(true),
            _ => panic!("Expected EpisodeDone error, got {:?}", result),
        }
    }

    #[test]
    fn test_physics_simulation() {
        let config = MountainCarConfig::default();
        let mut env = MountainCar::new(config.clone()).unwrap();

        let start_pos = -0.5;
        let start_vel = 0.0;
        env.state = Some(vec2(start_pos, start_vel));
        env.t = 0;

        let action = MixedItem::Discrete(2);

        let force = 1.0;
        let gravity = 0.0025;
        let mut v = start_vel + force * config.f - gravity * (3.0 * start_pos).cos();
        v = v.clamp(-config.max_v, config.max_v);
        let mut x = start_pos + v;
        x = x.clamp(config.min_x, config.max_x);

        let expected_state = vec2(x, v);

        let experience = env.step(action).unwrap();
        let diff = (experience.next_state - expected_state).norm();
        assert!(
            diff < 1e-6,
            "Physics mismatch. Expected {:?}, got {:?}",
            expected_state,
            experience.next_state
        );
    }

    #[test]
    fn test_physics_with_velocity_clamping() {
        let config = MountainCarConfig::default();
        let mut env = MountainCar::new(config.clone()).unwrap();

        env.state = Some(vec2(config.min_x, -0.05));

        let action = MixedItem::Discrete(1);

        let experience = env.step(action).unwrap();

        assert_eq!(experience.next_state[1], 0.0);
        assert_eq!(experience.next_state[0], config.min_x);
    }

    #[test]
    fn test_state_method_error() {
        let config = MountainCarConfig::default();
        let env = MountainCar::new(config).unwrap();

        match env.state() {
            Err(Error::NotInitialized) => assert!(true),
            _ => panic!("Expected NotInitialized error"),
        }
    }

    // MountainCarConfig tests
    #[test]
    fn test_mountain_car_config_default() {
        let config = MountainCarConfig::default();

        assert_eq!(config.f, 1e-3);
        assert_eq!(config.g, 25e-4);
        assert_eq!(config.min_x, -1.2);
        assert_eq!(config.max_x, 0.6);
        assert_eq!(config.max_v, 7e-2);
        assert_eq!(config.max_t, 200);
        assert_eq!(config.goal_x, 0.5);
        assert_eq!(config.goal_v, 0.0);
        assert_eq!(config.continuous, false);
        assert_eq!(config.reward, MountainCarReward::Constant);
    }

    #[test]
    fn test_mountain_car_config_new() {
        let config = MountainCarConfig::new();
        let default_config = MountainCarConfig::default();

        assert_eq!(config, default_config);
    }

    #[test]
    fn test_with_force() {
        let config = MountainCarConfig::default().with_force(0.5);

        assert_eq!(config.f, 0.5);
        assert_eq!(config.g, 25e-4);
    }

    #[test]
    fn test_with_gravity() {
        let config = MountainCarConfig::default().with_gravity(0.1);

        assert_eq!(config.g, 0.1);
        assert_eq!(config.f, 1e-3);
    }

    #[test]
    fn test_with_max_steps() {
        let config = MountainCarConfig::default().with_max_steps(500);

        assert_eq!(config.max_t, 500);
        assert_eq!(config.max_v, 7e-2);
    }

    #[test]
    fn test_with_min_position() {
        let config = MountainCarConfig::default().with_min_position(-2.0);

        assert_eq!(config.min_x, -2.0);
        assert_eq!(config.max_x, 0.6);
    }

    #[test]
    fn test_with_max_position() {
        let config = MountainCarConfig::default().with_max_position(1.0);

        assert_eq!(config.max_x, 1.0);
        assert_eq!(config.min_x, -1.2);
    }

    #[test]
    fn test_with_max_velocity() {
        let config = MountainCarConfig::default().with_max_velocity(0.1);

        assert_eq!(config.max_v, 0.1);
        assert_eq!(config.max_t, 200);
    }

    #[test]
    fn test_with_goal_position() {
        let config = MountainCarConfig::default().with_goal_position(0.8);

        assert_eq!(config.goal_x, 0.8);
        assert_eq!(config.goal_v, 0.0);
    }

    #[test]
    fn test_with_goal_velocity() {
        let config = MountainCarConfig::default().with_goal_velocity(0.5);

        assert_eq!(config.goal_v, 0.5);
        assert_eq!(config.goal_x, 0.5);
    }

    #[test]
    fn test_with_discrete_action() {
        let config = MountainCarConfig::default()
            .with_continuous_action()
            .with_discrete_action();

        assert_eq!(config.continuous, false);
        assert_eq!(config.f, 1e-3);
    }

    #[test]
    fn test_with_continuous_action() {
        let config = MountainCarConfig::default().with_continuous_action();

        assert_eq!(config.continuous, true);
        assert_eq!(config.f, 1e-3);
    }

    #[test]
    fn test_with_constant_reward() {
        let config = MountainCarConfig::default()
            .with_action_penalty_reward()
            .with_constant_reward();

        assert_eq!(config.reward, MountainCarReward::Constant);
        assert_eq!(config.f, 1e-3);
    }

    #[test]
    fn test_with_action_penalty_reward() {
        let config = MountainCarConfig::default().with_action_penalty_reward();

        assert_eq!(config.reward, MountainCarReward::ActionPenalty);
        assert_eq!(config.f, 1e-3);
    }

    #[test]
    fn test_mountain_car_reward_equality() {
        assert_eq!(MountainCarReward::Constant, MountainCarReward::Constant);
        assert_eq!(
            MountainCarReward::ActionPenalty,
            MountainCarReward::ActionPenalty
        );
        assert_ne!(
            MountainCarReward::Constant,
            MountainCarReward::ActionPenalty
        );
        assert_ne!(
            MountainCarReward::ActionPenalty,
            MountainCarReward::Constant
        );
    }

    #[test]
    fn test_mountain_car_config_clone() {
        let config1 = MountainCarConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_mountain_car_config_debug() {
        let config = MountainCarConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("MountainCarConfig"));
        assert!(debug_str.contains("f"));
        assert!(debug_str.contains("g"));
    }

    #[test]
    fn test_mountain_car_config_different_configs() {
        let config1 = MountainCarConfig::default();
        let config2 = MountainCarConfig::default().with_force(0.5);

        assert_ne!(config1, config2);
    }

    #[test]
    fn test_config_builder_pattern() {
        let config = MountainCarConfig::default()
            .with_force(0.01)
            .with_gravity(0.02)
            .with_max_steps(300)
            .with_min_position(-1.5)
            .with_max_position(0.8)
            .with_max_velocity(0.08)
            .with_goal_position(0.7)
            .with_goal_velocity(0.1)
            .with_continuous_action()
            .with_action_penalty_reward();

        assert_eq!(config.f, 0.01);
        assert_eq!(config.g, 0.02);
        assert_eq!(config.max_t, 300);
        assert_eq!(config.min_x, -1.5);
        assert_eq!(config.max_x, 0.8);
        assert_eq!(config.max_v, 0.08);
        assert_eq!(config.goal_x, 0.7);
        assert_eq!(config.goal_v, 0.1);
        assert_eq!(config.continuous, true);
        assert_eq!(config.reward, MountainCarReward::ActionPenalty);
    }
}
