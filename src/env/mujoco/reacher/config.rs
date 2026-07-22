#[derive(Debug, Clone)]
pub struct ReacherConfig {
    pub xml_file: String,
    pub frame_skip: u32,
    pub reward_dist_weight: f64,
    pub reward_control_weight: f64,
    pub observation_shape: (usize,), // Using tuple to match Python
    pub observation_dtype: String,   // Using string to represent dtype
    pub max_episode_steps: usize,
}

impl Default for ReacherConfig {
    fn default() -> Self {
        Self {
            xml_file: include_str!("model.xml").to_string(),
            frame_skip: 2,
            reward_dist_weight: 1.0,
            reward_control_weight: 1.0,
            observation_shape: (10,), // cos(theta)[2] + sin(theta)[2] + target qpos[2] + arm qvel[2] + fingertip-target vec[2]
            observation_dtype: "f64".to_string(), // Representing numpy.float64
            max_episode_steps: 50,
        }
    }
}
