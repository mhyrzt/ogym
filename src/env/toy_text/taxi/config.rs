#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaxiConfig {
    pub max_episode_steps: usize,
}

impl Default for TaxiConfig {
    fn default() -> Self {
        Self {
            max_episode_steps: 200,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        assert_eq!(TaxiConfig::default().max_episode_steps, 200);
    }
}
