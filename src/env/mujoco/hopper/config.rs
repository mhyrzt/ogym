
#[derive(Debug, Clone)]
pub struct HopperConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub forward_reward_weight: f64,
    pub ctrl_cost_weight: f64,
    pub healthy_reward: f64,
    pub terminate_when_unhealthy: bool,
    pub healthy_state_range: (f64, f64),
    pub healthy_z_range: (f64, f64),
    pub healthy_angle_range: (f64, f64),
    pub reset_noise_scale: f64,
    pub exclude_current_positions_from_observation: bool,
}

impl Default for HopperConfig {
    fn default() -> Self {
        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 4,
            forward_reward_weight: 1.0,
            ctrl_cost_weight: 1e-3,
            healthy_reward: 1.0,
            terminate_when_unhealthy: true,
            healthy_state_range: (-100.0, 100.0),
            healthy_z_range: (0.7, f64::INFINITY), // Using f64::INFINITY instead of float("inf")
            healthy_angle_range: (-0.2, 0.2),
            reset_noise_scale: 5e-3,
            exclude_current_positions_from_observation: true,
        }
    }
}
