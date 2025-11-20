use ogym::env::environment::{Experience, Terminal};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_creation() {
        let experience = Experience::new(
            10,           // curr_state
            5.5,          // reward
            2,            // action
            15,           // next_state
            "info".to_string(), // info
            Terminal::Ongoing,  // terminal
            1,            // step
        );

        assert_eq!(experience.curr_state, 10);
        assert_eq!(experience.action, 2);
        assert_eq!(experience.reward, 5.5);
        assert_eq!(experience.next_state, 15);
        assert_eq!(experience.info, "info");
        assert_eq!(experience.terminal, Terminal::Ongoing);
        assert_eq!(experience.step, 1);
    }

    #[test]
    fn test_experience_debug_formatting() {
        let experience = Experience::new(
            5,
            3.0,
            1,
            6,
            "test_info".to_string(),
            Terminal::Terminate,
            0,
        );

        let debug_str = format!("{:?}", experience);
        assert!(debug_str.contains("Experience"));
        assert!(debug_str.contains("curr_state"));
        assert!(debug_str.contains("action"));
    }

    #[test]
    fn test_experience_clone_and_copy() {
        let experience1 = Experience::new(
            10,
            5.5,
            2,
            15,
            "info".to_string(),
            Terminal::Ongoing,
            1,
        );

        // Since Experience derives Copy, we can copy it directly
        let experience2 = experience1;

        // Both should have the same values
        assert_eq!(experience1.curr_state, experience2.curr_state);
        assert_eq!(experience1.action, experience2.action);
        assert_eq!(experience1.reward, experience2.reward);
        assert_eq!(experience1.next_state, experience2.next_state);
        assert_eq!(experience1.info, experience2.info);
        assert_eq!(experience1.terminal, experience2.terminal);
        assert_eq!(experience1.step, experience2.step);
    }

    #[test]
    fn test_experience_different_terminal_states() {
        // Test with different terminal states
        let exp_ongoing = Experience::new(0, 1.0, 1, 1, "info".to_string(), Terminal::Ongoing, 1);
        let exp_terminate = Experience::new(0, 1.0, 1, 1, "info".to_string(), Terminal::Terminate, 1);
        let exp_truncate = Experience::new(0, 1.0, 1, 1, "info".to_string(), Terminal::Truncate, 1);
        let exp_both = Experience::new(0, 1.0, 1, 1, "info".to_string(), Terminal::Both, 1);

        assert_eq!(exp_ongoing.terminal, Terminal::Ongoing);
        assert_eq!(exp_terminate.terminal, Terminal::Terminate);
        assert_eq!(exp_truncate.terminal, Terminal::Truncate);
        assert_eq!(exp_both.terminal, Terminal::Both);
    }

    #[test]
    fn test_experience_different_values() {
        // Create experiences with different values to ensure fields are properly stored
        let exp1 = Experience::new(100, 10.0, 5, 105, "info1".to_string(), Terminal::Ongoing, 10);
        let exp2 = Experience::new(200, 20.0, 8, 208, "info2".to_string(), Terminal::Terminate, 20);

        assert_eq!(exp1.curr_state, 100);
        assert_eq!(exp1.action, 5);
        assert_eq!(exp1.reward, 10.0);
        assert_eq!(exp1.next_state, 105);
        assert_eq!(exp1.info, "info1");
        assert_eq!(exp1.terminal, Terminal::Ongoing);
        assert_eq!(exp1.step, 10);

        assert_eq!(exp2.curr_state, 200);
        assert_eq!(exp2.action, 8);
        assert_eq!(exp2.reward, 20.0);
        assert_eq!(exp2.next_state, 208);
        assert_eq!(exp2.info, "info2");
        assert_eq!(exp2.terminal, Terminal::Terminate);
        assert_eq!(exp2.step, 20);
    }

    #[test]
    fn test_experience_equality() {
        let exp1 = Experience::new(5, 2.5, 1, 6, "info".to_string(), Terminal::Ongoing, 1);
        let exp2 = Experience::new(5, 2.5, 1, 6, "info".to_string(), Terminal::Ongoing, 1);

        // Since Experience implements PartialEq, we can compare them directly
        assert_eq!(exp1, exp2);
    }

    #[test]
    fn test_experience_with_negative_values() {
        let experience = Experience::new(
            -5,           // curr_state
            -2.5,         // reward (can be negative)
            -1,           // action
            -6,           // next_state
            "negative_info".to_string(), // info
            Terminal::Truncate,  // terminal
            100,          // step
        );

        assert_eq!(experience.curr_state, -5);
        assert_eq!(experience.action, -1);
        assert_eq!(experience.reward, -2.5);
        assert_eq!(experience.next_state, -6);
        assert_eq!(experience.info, "negative_info");
        assert_eq!(experience.terminal, Terminal::Truncate);
        assert_eq!(experience.step, 100);
    }

    #[test]
    fn test_experience_with_large_values() {
        let experience = Experience::new(
            i32::MAX,
            f64::MAX,
            i32::MAX,
            i32::MAX,
            "large_info".to_string(),
            Terminal::Both,
            u32::MAX,
        );

        assert_eq!(experience.curr_state, i32::MAX);
        assert_eq!(experience.action, i32::MAX);
        assert_eq!(experience.reward, f64::MAX);
        assert_eq!(experience.next_state, i32::MAX);
        assert_eq!(experience.info, "large_info");
        assert_eq!(experience.terminal, Terminal::Both);
        assert_eq!(experience.step, u32::MAX);
    }
}