use ogym::env::environment::Error;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_invalid_max_step_error() {
        let error = Error::InvalidMaxStep;
        assert_eq!(error.to_string(), "Invalid Max Step");
    }

    #[test]
    fn test_invalid_action_error() {
        let error = Error::InvalidAction;
        assert_eq!(error.to_string(), "Invalid Action");
    }

    #[test]
    fn test_episode_done_error() {
        let error = Error::EpisodeDone;
        assert_eq!(
            error.to_string(),
            "Cannot take action: the environment is done. Call reset() to start a new episode."
        );
    }

    #[test]
    fn test_not_initialized_error() {
        let error = Error::NotInitialized;
        assert_eq!(
            error.to_string(),
            "Environment has not been initialized. Call reset() before stepping."
        );
    }

    #[test]
    fn test_mj_init_error() {
        let error = Error::MjInitError("test error".to_string());
        assert_eq!(error.to_string(), "MuJoCo Initialization Error: test error");
    }

    #[test]
    fn test_invalid_action_dimension_error() {
        let error = Error::InvalidActionDimension {
            expected: 4,
            got: 3,
        };
        assert_eq!(
            error.to_string(),
            "Action dimension mismatch. Expected 4, got 3"
        );
    }

    #[test]
    fn test_invalid_state_dimension_error() {
        let error = Error::InvalidStateDimension {
            field: "position",
            expected: 2,
            got: 3,
        };
        assert_eq!(
            error.to_string(),
            "State dimension mismatch for position. Expected 2, got 3"
        );
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = Error::InvalidMaxStep;
        let std_error: &dyn StdError = &error;
        assert!(std_error.source().is_none());
    }

    #[test]
    fn test_error_derives() {
        let error1 = Error::InvalidAction;

        let debug_str = format!("{:?}", error1);
        assert!(debug_str.contains("InvalidAction"));

        let cloned_error = error1.clone();
        assert!(matches!(cloned_error, Error::InvalidAction));
    }
}
