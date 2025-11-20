
#[derive(Debug, Clone)]
pub struct SwimmerConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub forward_reward_weight: f64,
    pub ctrl_cost_weight: f64,
    pub reset_noise_scale: f64,
    pub exclude_current_positions_from_observation: bool,
}

impl Default for SwimmerConfig {
    fn default() -> Self {
        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 4,
            forward_reward_weight: 1.0,
            ctrl_cost_weight: 1e-4,
            reset_noise_scale: 0.1,
            exclude_current_positions_from_observation: true,
        }
    }
}
