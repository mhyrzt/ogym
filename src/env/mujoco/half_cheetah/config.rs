#[derive(Debug, Clone)]
pub struct HalfCheetahConfig {
    xml: String,
    frame_skip: u32,
    forward_reward_weight: f64,
    ctrl_cost_weight: f64,
    reset_noise_scale: f64,
    exclude_current_positions_from_observation: bool,
}

impl Default for HalfCheetahConfig {
    fn default() -> Self {
        Self {
            xml: include_str!("model.xml").to_string(),
            frame_skip: 5,
            forward_reward_weight: 1.0,
            ctrl_cost_weight: 0.1,
            reset_noise_scale: 0.1,
            exclude_current_positions_from_observation: true,
        }
    }
}

impl HalfCheetahConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_xml(mut self, xml: impl Into<String>) -> Self {
        self.xml = xml.into();
        self
    }

    pub fn with_frame_skip(mut self, frame_skip: u32) -> Self {
        assert!(frame_skip > 0, "frame_skip must be greater than 0");
        self.frame_skip = frame_skip;
        self
    }

    pub fn with_forward_reward_weight(mut self, weight: f64) -> Self {
        self.forward_reward_weight = weight;
        self
    }

    pub fn with_ctrl_cost_weight(mut self, weight: f64) -> Self {
        self.ctrl_cost_weight = weight;
        self
    }

    pub fn with_reset_noise_scale(mut self, scale: f64) -> Self {
        assert!(scale >= 0.0, "noise scale cannot be negative");
        self.reset_noise_scale = scale;
        self
    }

    pub fn with_position_exclusion(mut self, exclude: bool) -> Self {
        self.exclude_current_positions_from_observation = exclude;
        self
    }

    pub fn xml(&self) -> &str {
        &self.xml
    }

    pub fn frame_skip(&self) -> u32 {
        self.frame_skip
    }

    pub fn forward_reward_weight(&self) -> f64 {
        self.forward_reward_weight
    }

    pub fn ctrl_cost_weight(&self) -> f64 {
        self.ctrl_cost_weight
    }

    pub fn reset_noise_scale(&self) -> f64 {
        self.reset_noise_scale
    }

    pub fn exclude_current_positions_from_observation(&self) -> bool {
        self.exclude_current_positions_from_observation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let config = HalfCheetahConfig::new();
        assert_eq!(config.frame_skip(), 5);
        assert_eq!(config.reset_noise_scale(), 0.1);
    }

    #[test]
    fn test_builder_customization() {
        let config = HalfCheetahConfig::new()
            .with_frame_skip(10)
            .with_reset_noise_scale(0.5)
            .with_forward_reward_weight(2.0);

        assert_eq!(config.frame_skip(), 10);
        assert!((config.reset_noise_scale() - 0.5).abs() < f64::EPSILON);
        assert!((config.forward_reward_weight() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "frame_skip must be greater than 0")]
    fn test_invalid_frame_skip() {
        HalfCheetahConfig::new().with_frame_skip(0);
    }

    #[test]
    #[should_panic(expected = "noise scale cannot be negative")]
    fn test_invalid_noise_scale() {
        HalfCheetahConfig::new().with_reset_noise_scale(-0.1);
    }
}
