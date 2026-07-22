#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlackjackConfig {
    /// Pay 1.5x reward for a natural (2-card 21) win instead of 1x.
    pub natural: bool,
}

impl Default for BlackjackConfig {
    fn default() -> Self {
        Self { natural: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        assert!(!BlackjackConfig::default().natural);
    }
}
