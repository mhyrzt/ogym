#[derive(Debug, Clone, PartialEq)]
pub struct FrozenLakeConfig {
    /// Row-major grid, each row a byte string: b'S' start, b'F' frozen,
    /// b'H' hole, b'G' goal.
    pub map: Vec<Vec<u8>>,
    pub is_slippery: bool,
    pub max_episode_steps: usize,
}

impl Default for FrozenLakeConfig {
    fn default() -> Self {
        Self {
            map: ["SFFF", "FHFH", "FFFH", "HFFG"]
                .iter()
                .map(|row| row.bytes().collect())
                .collect(),
            is_slippery: true,
            max_episode_steps: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_map_is_4x4_with_one_start_and_goal() {
        let config = FrozenLakeConfig::default();
        assert_eq!(config.map.len(), 4);
        assert!(config.map.iter().all(|row| row.len() == 4));
        let starts = config.map.iter().flatten().filter(|&&c| c == b'S').count();
        let goals = config.map.iter().flatten().filter(|&&c| c == b'G').count();
        assert_eq!(starts, 1);
        assert_eq!(goals, 1);
        assert!(config.is_slippery);
        assert_eq!(config.max_episode_steps, 100);
    }
}
