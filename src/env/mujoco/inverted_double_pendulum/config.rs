use nalgebra::DVector;

#[derive(Debug, Clone)]
pub struct InvertedDoublePendulumConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub healthy_reward: f64,
    pub reset_noise_scale: f64,
    pub observation_shape: (usize,), // Using tuple to match Python
    pub observation_dtype: String, // Using string to represent dtype
    pub max_steps: usize,
}

impl Default for InvertedDoublePendulumConfig {
    fn default() -> Self {
        Self {
            xml_file: "model.xml".to_string(),
            frame_skip: 5,
            healthy_reward: 10.0,
            reset_noise_scale: 0.1,
            observation_shape: (9,), // 9 elements based on env
            observation_dtype: "f64".to_string(), // Representing numpy.float64
            max_steps: 1000,
        }
    }
}