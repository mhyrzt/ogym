use std::f64;

#[derive(Debug, Clone)]
pub struct HumanoidConfig {
    pub(crate) xml: String,
    pub(crate) frame_skip: u32,
    pub(crate) forward_reward_weight: f64,
    pub(crate) ctrl_cost_weight: f64,
    pub(crate) contact_cost_weight: f64,
    pub(crate) contact_cost_range: (f64, f64),
    pub(crate) healthy_reward: f64,
    pub(crate) terminate_when_unhealthy: bool,
    pub(crate) healthy_z_range: (f64, f64),
    pub(crate) reset_noise_scale: f64,
    pub(crate) exclude_current_positions_from_observation: bool,
    pub(crate) include_cinert_in_observation: bool,
    pub(crate) include_cvel_in_observation: bool,
    pub(crate) include_qfrc_actuator_in_observation: bool,
    pub(crate) include_cfrc_ext_in_observation: bool,
    pub(crate) max_episode_steps: usize,
}

impl Default for HumanoidConfig {
    fn default() -> Self {
        Self {
            xml: include_str!("model.xml").to_string(),
            frame_skip: 5,
            forward_reward_weight: 1.25,
            ctrl_cost_weight: 0.1,
            contact_cost_weight: 5e-7,
            contact_cost_range: (f64::NEG_INFINITY, 10.0),
            healthy_reward: 5.0,
            terminate_when_unhealthy: true,
            healthy_z_range: (1.0, 2.0),
            reset_noise_scale: 1e-2,
            exclude_current_positions_from_observation: true,
            include_cinert_in_observation: true,
            include_cvel_in_observation: true,
            include_qfrc_actuator_in_observation: true,
            include_cfrc_ext_in_observation: true,
            max_episode_steps: 1000,
        }
    }
}

impl HumanoidConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_xml(mut self, xml: impl Into<String>) -> Self {
        self.xml = xml.into();
        self
    }

    pub fn with_frame_skip(mut self, frame_skip: u32) -> Self {
        self.frame_skip = frame_skip;
        self
    }

    pub fn with_reward_weights(
        mut self,
        forward_weight: f64,
        ctrl_cost_weight: f64,
        contact_cost_weight: f64,
        healthy_reward: f64,
    ) -> Self {
        self.forward_reward_weight = forward_weight;
        self.ctrl_cost_weight = ctrl_cost_weight;
        self.contact_cost_weight = contact_cost_weight;
        self.healthy_reward = healthy_reward;
        self
    }

    pub fn with_contact_cost_range(mut self, min: f64, max: f64) -> Self {
        self.contact_cost_range = (min, max);
        self
    }

    pub fn with_healthy_z_range(mut self, min: f64, max: f64) -> Self {
        self.healthy_z_range = (min, max);
        self
    }

    pub fn with_reset_noise_scale(mut self, scale: f64) -> Self {
        self.reset_noise_scale = scale;
        self
    }

    pub fn with_termination_condition(mut self, terminate_when_unhealthy: bool) -> Self {
        self.terminate_when_unhealthy = terminate_when_unhealthy;
        self
    }

    pub fn with_max_episode_steps(mut self, steps: usize) -> Self {
        self.max_episode_steps = steps;
        self
    }

    pub fn with_observation_settings(
        mut self,
        exclude_pos: bool,
        include_cinert: bool,
        include_cvel: bool,
        include_qfrc: bool,
        include_cfrc: bool,
    ) -> Self {
        self.exclude_current_positions_from_observation = exclude_pos;
        self.include_cinert_in_observation = include_cinert;
        self.include_cvel_in_observation = include_cvel;
        self.include_qfrc_actuator_in_observation = include_qfrc;
        self.include_cfrc_ext_in_observation = include_cfrc;
        self
    }

    pub fn xml(&self) -> &str {
        &self.xml
    }

    pub fn frame_skip(&self) -> u32 {
        self.frame_skip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let config = HumanoidConfig::new()
            .with_reset_noise_scale(0.5)
            .with_frame_skip(10)
            .with_reward_weights(2.0, 0.5, 0.01, 10.0);

        assert_eq!(config.reset_noise_scale, 0.5);
        assert_eq!(config.frame_skip, 10);
        assert_eq!(config.forward_reward_weight, 2.0);
        assert_eq!(config.terminate_when_unhealthy, true);
    }

    #[test]
    fn test_default_values() {
        let config = HumanoidConfig::default();
        assert_eq!(config.frame_skip, 5);
        assert!((config.healthy_reward - 5.0).abs() < f64::EPSILON);
    }
}
