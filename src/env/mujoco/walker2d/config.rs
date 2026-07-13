#[derive(Debug, Clone)]
pub struct Walker2dConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub forward_reward_weight: f64,
    pub ctrl_cost_weight: f64,
    pub healthy_reward: f64,
    pub terminate_when_unhealthy: bool,
    pub healthy_z_range: (f64, f64),
    pub healthy_angle_range: (f64, f64),
    pub reset_noise_scale: f64,
    pub exclude_current_positions_from_observation: bool,
    pub max_episode_steps: usize,
}

impl Default for Walker2dConfig {
    fn default() -> Self {
        Self {
            xml_file: include_str!("model.xml").to_string(),
            frame_skip: 4,
            forward_reward_weight: 1.0,
            ctrl_cost_weight: 1e-3,
            healthy_reward: 1.0,
            terminate_when_unhealthy: true,
            healthy_z_range: (0.8, 2.0),
            healthy_angle_range: (-1.0, 1.0),
            reset_noise_scale: 5e-3,
            exclude_current_positions_from_observation: true,
            max_episode_steps: 1000,
        }
    }
}
