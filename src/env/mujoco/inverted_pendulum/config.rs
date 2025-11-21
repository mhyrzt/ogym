#[derive(Debug, Clone)]
pub struct InvertedPendulumConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub reset_noise_scale: f64,
    pub observation_shape: (usize,), // Using tuple to match Python
    pub observation_low: f64,
    pub observation_high: f64,
    pub default_camera_config: std::collections::HashMap<String, f64>,
}

impl Default for InvertedPendulumConfig {
    fn default() -> Self {
        let mut default_camera_config = std::collections::HashMap::new();
        default_camera_config.insert("trackbodyid".to_string(), 0.0);
        default_camera_config.insert("distance".to_string(), 2.04);

        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 2,
            reset_noise_scale: 0.01,
            observation_shape: (4,),
            observation_low: f64::NEG_INFINITY,
            observation_high: f64::INFINITY,
            default_camera_config,
        }
    }
}
