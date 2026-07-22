#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CliffWalkingConfig {
    pub nrow: usize,
    pub ncol: usize,
    pub max_episode_steps: usize,
}

impl Default for CliffWalkingConfig {
    fn default() -> Self {
        Self {
            nrow: 4,
            ncol: 12,
            max_episode_steps: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_grid_is_4x12() {
        let config = CliffWalkingConfig::default();
        assert_eq!(config.nrow, 4);
        assert_eq!(config.ncol, 12);
        assert_eq!(config.max_episode_steps, 100);
    }
}
