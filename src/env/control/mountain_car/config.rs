#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MountainCarReward {
    Constant,
    ActionPenalty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountainCarConfig {
    pub f: f64,
    pub g: f64,
    pub max_t: u32,
    pub min_x: f64,
    pub max_x: f64,
    pub max_v: f64,
    pub goal_x: f64,
    pub goal_v: f64,
    pub continuous: bool,
    pub reward: MountainCarReward,
}

impl Default for MountainCarConfig {
    fn default() -> Self {
        MountainCarConfig {
            f: 1e-3,
            g: 25e-4,
            min_x: -1.2,
            max_x: 0.6,
            max_v: 7e-2,
            max_t: 200,
            goal_x: 0.5,
            goal_v: 0.0,
            continuous: false,
            reward: MountainCarReward::Constant,
        }
    }
}

impl MountainCarConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_force(mut self, f: f64) -> Self {
        self.f = f;
        self
    }

    pub fn with_gravity(mut self, g: f64) -> Self {
        self.g = g;
        self
    }

    pub fn with_max_steps(mut self, max_t: u32) -> Self {
        self.max_t = max_t;
        self
    }

    pub fn with_min_position(mut self, min_x: f64) -> Self {
        self.min_x = min_x;
        self
    }

    pub fn with_max_position(mut self, max_x: f64) -> Self {
        self.max_x = max_x;
        self
    }

    pub fn with_max_velocity(mut self, max_v: f64) -> Self {
        self.max_v = max_v;
        self
    }

    pub fn with_goal_position(mut self, goal_x: f64) -> Self {
        self.goal_x = goal_x;
        self
    }

    pub fn with_goal_velocity(mut self, goal_v: f64) -> Self {
        self.goal_v = goal_v;
        self
    }

    pub fn with_discrete_action(mut self) -> Self {
        self.continuous = false;
        self
    }

    pub fn with_continuous_action(mut self) -> Self {
        self.continuous = true;
        self
    }

    pub fn with_constant_reward(mut self) -> Self {
        self.reward = MountainCarReward::Constant;
        self
    }

    pub fn with_action_penalty_reward(mut self) -> Self {
        self.reward = MountainCarReward::ActionPenalty;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
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
    fn test_new_config() {
        let config = MountainCarConfig::new();
        let default_config = MountainCarConfig::default();
        assert_eq!(config, default_config);
    }

    #[test]
    fn test_builder_methods() {
        let config = MountainCarConfig::new()
            .with_force(0.002)
            .with_gravity(0.003)
            .with_max_steps(500)
            .with_min_position(-2.0)
            .with_max_position(1.0)
            .with_max_velocity(0.1)
            .with_goal_position(0.8)
            .with_goal_velocity(0.05);

        assert_eq!(config.f, 0.002);
        assert_eq!(config.g, 0.003);
        assert_eq!(config.max_t, 500);
        assert_eq!(config.min_x, -2.0);
        assert_eq!(config.max_x, 1.0);
        assert_eq!(config.max_v, 0.1);
        assert_eq!(config.goal_x, 0.8);
        assert_eq!(config.goal_v, 0.05);
    }

    #[test]
    fn test_continuous_action_toggle() {
        let config = MountainCarConfig::new().with_continuous_action();
        assert!(config.continuous);

        let config = config.with_discrete_action();
        assert!(!config.continuous);
    }

    #[test]
    fn test_reward_toggle() {
        let config = MountainCarConfig::new().with_action_penalty_reward();
        assert_eq!(config.reward, MountainCarReward::ActionPenalty);

        let config = config.with_constant_reward();
        assert_eq!(config.reward, MountainCarReward::Constant);
    }

    #[test]
    fn test_clone_and_debug() {
        let config = MountainCarConfig::default();
        let cloned_config = config.clone();
        assert_eq!(config, cloned_config);

        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());

        let reward = MountainCarReward::Constant;
        let cloned_reward = reward.clone();
        assert_eq!(reward, cloned_reward);

        let debug_reward = format!("{:?}", reward);
        assert!(!debug_reward.is_empty());
    }
}
