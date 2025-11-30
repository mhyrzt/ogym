
#[derive(Debug, Clone)]
pub struct PusherConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub reward_near_weight: f64,
    pub reward_dist_weight: f64,
    pub reward_control_weight: f64,
    pub observation_shape: (usize,), // Using tuple to match Python
    pub observation_dtype: String,   // Using string to represent dtype
}

impl Default for PusherConfig {
    fn default() -> Self {
        Self {
            xml_file: include_str!("model.xml").to_string(),
            frame_skip: 5,
            reward_near_weight: 0.5,
            reward_dist_weight: 1.0,
            reward_control_weight: 0.1,
            observation_shape: (23,),             // 23 elements based on env
            observation_dtype: "f64".to_string(), // Representing numpy.float64
        }
    }
}
