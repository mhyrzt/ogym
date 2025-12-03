use std::fmt;

#[derive(Clone)]
pub struct AntConfig {
    pub(crate) xml: String,
    pub(crate) frame_skip: u32,
    pub(crate) forward_reward_weight: f64,
    pub(crate) ctrl_cost_weight: f64,
    pub(crate) contact_cost_weight: f64,
    pub(crate) healthy_reward: f64,
    pub(crate) main_body: u32,
    pub(crate) terminate_when_unhealthy: bool,
    pub(crate) healthy_z_range: (f64, f64),
    pub(crate) contact_force_range: (f64, f64),
    pub(crate) reset_noise_scale: f64,
    pub(crate) exclude_current_positions_from_observation: bool,
    pub(crate) include_cfrc_ext_in_observation: bool,
    pub(crate) max_episode_steps: usize,
}

impl AntConfig {
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

    pub fn with_forward_reward_weight(mut self, weight: f64) -> Self {
        self.forward_reward_weight = weight;
        self
    }

    pub fn with_cost_weights(mut self, ctrl_weight: f64, contact_weight: f64) -> Self {
        self.ctrl_cost_weight = ctrl_weight;
        self.contact_cost_weight = contact_weight;
        self
    }

    pub fn with_healthy_reward(mut self, reward: f64) -> Self {
        self.healthy_reward = reward;
        self
    }

    pub fn with_reset_noise_scale(mut self, scale: f64) -> Self {
        self.reset_noise_scale = scale;
        self
    }

    pub fn with_termination_on_unhealthy(mut self, terminate: bool) -> Self {
        self.terminate_when_unhealthy = terminate;
        self
    }

    pub fn with_healthy_z_range(mut self, min: f64, max: f64) -> Self {
        self.healthy_z_range = (min, max);
        self
    }

    pub fn with_max_episode_steps(mut self, steps: usize) -> Self {
        self.max_episode_steps = steps;
        self
    }

    pub fn with_exclude_current_positions_from_observation(mut self, exclude: bool) -> Self {
        self.exclude_current_positions_from_observation = exclude;
        self
    }

    pub fn with_include_cfrc_ext_in_observation(mut self, include: bool) -> Self {
        self.include_cfrc_ext_in_observation = include;
        self
    }

    pub fn with_observation_settings(mut self, exclude_pos: bool, include_cfrc: bool) -> Self {
        self.exclude_current_positions_from_observation = exclude_pos;
        self.include_cfrc_ext_in_observation = include_cfrc;
        self
    }

    pub fn xml(&self) -> &str {
        &self.xml
    }
    pub fn reset_noise_scale(&self) -> f64 {
        self.reset_noise_scale
    }
}

impl Default for AntConfig {
    fn default() -> Self {
        Self {
            xml: include_str!("model.xml").to_string(),
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
            max_episode_steps: 1000,
        }
    }
}

impl fmt::Debug for AntConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AntConfig")
            .field("frame_skip", &self.frame_skip)
            .field("forward_reward_weight", &self.forward_reward_weight)
            .field("ctrl_cost_weight", &self.ctrl_cost_weight)
            .field("healthy_reward", &self.healthy_reward)
            .field("reset_noise_scale", &self.reset_noise_scale)
            .field(
                "exclude_pos",
                &self.exclude_current_positions_from_observation,
            )
            .field("include_cfrc", &self.include_cfrc_ext_in_observation)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let config = AntConfig::new()
            .with_frame_skip(10)
            .with_healthy_reward(5.0)
            .with_reset_noise_scale(0.2)
            .with_include_cfrc_ext_in_observation(false);

        assert_eq!(config.frame_skip, 10);
        assert!((config.healthy_reward - 5.0).abs() < 1e-6);
        assert!((config.reset_noise_scale - 0.2).abs() < 1e-6);
        assert!(!config.include_cfrc_ext_in_observation);
        assert_eq!(config.ctrl_cost_weight, 0.5);
    }
}
