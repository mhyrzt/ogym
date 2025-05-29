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
    pub fn builder() -> Self {
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
