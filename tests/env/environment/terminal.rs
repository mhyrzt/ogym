use ogym::env::environment::Terminal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_enum_variants() {
        assert_eq!(format!("{:?}", Terminal::Ongoing), "Ongoing");
        assert_eq!(format!("{:?}", Terminal::Both), "Both");
        assert_eq!(format!("{:?}", Terminal::Truncate), "Truncate");
        assert_eq!(format!("{:?}", Terminal::Terminate), "Terminate");
    }

    #[test]
    fn test_terminal_is_terminated() {
        assert_eq!(Terminal::Ongoing.is_terminated(), false);
        assert_eq!(Terminal::Truncate.is_terminated(), false);
        assert_eq!(Terminal::Terminate.is_terminated(), true);
        assert_eq!(Terminal::Both.is_terminated(), true);
    }

    #[test]
    fn test_terminal_is_truncated() {
        assert_eq!(Terminal::Ongoing.is_truncated(), false);
        assert_eq!(Terminal::Truncate.is_truncated(), true);
        assert_eq!(Terminal::Terminate.is_truncated(), false);
        assert_eq!(Terminal::Both.is_truncated(), true);
    }

    #[test]
    fn test_terminal_is_done() {
        assert_eq!(Terminal::Ongoing.is_done(), false);
        assert_eq!(Terminal::Truncate.is_done(), true);
        assert_eq!(Terminal::Terminate.is_done(), true);
        assert_eq!(Terminal::Both.is_done(), true);
    }

    #[test]
    fn test_terminal_from_flags() {
        // Test all possible flag combinations
        assert_eq!(Terminal::from_flags(false, false), Terminal::Ongoing);
        assert_eq!(Terminal::from_flags(true, false), Terminal::Terminate);
        assert_eq!(Terminal::from_flags(false, true), Terminal::Truncate);
        assert_eq!(Terminal::from_flags(true, true), Terminal::Both);
    }

    #[test]
    fn test_terminal_clone_and_copy() {
        let terminal = Terminal::Terminate;
        let cloned_terminal = terminal.clone();
        assert_eq!(terminal, cloned_terminal);
        assert_eq!(terminal, Terminal::Terminate);
    }

    #[test]
    fn test_terminal_eq_and_ne() {
        assert_eq!(Terminal::Ongoing, Terminal::Ongoing);
        assert_ne!(Terminal::Ongoing, Terminal::Terminate);
        assert_ne!(Terminal::Truncate, Terminal::Terminate);
        assert_eq!(Terminal::Both, Terminal::Both);
    }

    #[test]
    fn test_terminal_debug_formatting() {
        assert_eq!(format!("{:?}", Terminal::Ongoing), "Ongoing");
        assert_eq!(format!("{:?}", Terminal::Truncate), "Truncate");
        assert_eq!(format!("{:?}", Terminal::Terminate), "Terminate");
        assert_eq!(format!("{:?}", Terminal::Both), "Both");
    }

    #[test]
    fn test_terminal_comprehensive_scenarios() {
        // Test a scenario where we transition from Ongoing to various states
        let ongoing = Terminal::Ongoing;
        assert_eq!(ongoing.is_done(), false);
        assert_eq!(ongoing.is_terminated(), false);
        assert_eq!(ongoing.is_truncated(), false);

        let terminated = Terminal::from_flags(true, false);
        assert_eq!(terminated, Terminal::Terminate);
        assert_eq!(terminated.is_done(), true);
        assert_eq!(terminated.is_terminated(), true);
        assert_eq!(terminated.is_truncated(), false);

        let truncated = Terminal::from_flags(false, true);
        assert_eq!(truncated, Terminal::Truncate);
        assert_eq!(truncated.is_done(), true);
        assert_eq!(truncated.is_terminated(), false);
        assert_eq!(truncated.is_truncated(), true);

        let both = Terminal::from_flags(true, true);
        assert_eq!(both, Terminal::Both);
        assert_eq!(both.is_done(), true);
        assert_eq!(both.is_terminated(), true);
        assert_eq!(both.is_truncated(), true);
    }
}