use nalgebra::DVector;

#[derive(Debug, Clone)]
pub struct HumanoidStandupConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub uph_cost_weight: f64,
    pub ctrl_cost_weight: f64,
    pub impact_cost_weight: f64,
    pub impact_cost_range: (f64, f64),
    pub reset_noise_scale: f64,
    pub exclude_current_positions_from_observation: bool,
    pub include_cinert_in_observation: bool,
    pub include_cvel_in_observation: bool,
    pub include_qfrc_actuator_in_observation: bool,
    pub include_cfrc_ext_in_observation: bool,
}

impl Default for HumanoidStandupConfig {
    fn default() -> Self {
        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 5,
            uph_cost_weight: 1.0,
            ctrl_cost_weight: 0.1,
            impact_cost_weight: 0.5e-6,
            impact_cost_range: (f64::NEG_INFINITY, 10.0),
            reset_noise_scale: 1e-2,
            exclude_current_positions_from_observation: true,
            include_cinert_in_observation: true,
            include_cvel_in_observation: true,
            include_qfrc_actuator_in_observation: true,
            include_cfrc_ext_in_observation: true,
        }
    }
}
