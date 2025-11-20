use nalgebra::DVector;

#[derive(Debug, Clone)]
pub struct AntConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub forward_reward_weight: f64,
    pub ctrl_cost_weight: f64,
    pub contact_cost_weight: f64,
    pub healthy_reward: f64,
    pub main_body: u32, // Using u32 instead of int | str union type
    pub terminate_when_unhealthy: bool,
    pub healthy_z_range: (f64, f64),
    pub contact_force_range: (f64, f64),
    pub reset_noise_scale: f64,
    pub exclude_current_positions_from_observation: bool,
    pub include_cfrc_ext_in_observation: bool,
}

impl Default for AntConfig {
    fn default() -> Self {
        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 5,
            forward_reward_weight: 1.0,
            ctrl_cost_weight: 0.5,
            contact_cost_weight: 5e-4,
            healthy_reward: 1.0,
            main_body: 1,
            terminate_when_unhealthy: true,
            healthy_z_range: (0.2, 1.0),
            contact_force_range: (-1.0, 1.0),
            reset_noise_scale: 0.1,
            exclude_current_positions_from_observation: true,
            include_cfrc_ext_in_observation: true,
        }
    }
}
