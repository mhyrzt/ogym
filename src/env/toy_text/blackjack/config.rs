#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BlackjackConfig {
    /// Pay 1.5x reward for a natural (2-card 21) win instead of 1x. Ignored
    /// when `sab` is set, matching Gymnasium's precedence.
    pub natural: bool,
    /// Sutton & Barto rules: a player natural (when the dealer doesn't also
    /// have one) is an outright win worth 1.0, with no 1.5x bonus.
    pub sab: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        assert!(!BlackjackConfig::default().natural);
        assert!(!BlackjackConfig::default().sab);
    }
}
