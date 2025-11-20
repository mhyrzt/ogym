use ogym::env::environment::Error;
use ogym::spaces::Error as SpaceError;

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
    fn test_space_error() {
        let space_error = SpaceError::InvalidSample; // Assuming this is a valid variant
        let error = Error::Space(space_error);
        assert!(error.to_string().contains("Space error:"));
    }

    #[test]
    fn test_episode_done_error() {
        let error = Error::EpisodeDone;
        assert_eq!(error.to_string(), "Cannot take action: the environment is done. Call reset() to start a new episode.");
    }

    #[test]
    fn test_not_initialized_error() {
        let error = Error::NotInitialized;
        assert_eq!(error.to_string(), "Environment has not been initialized. Call reset() before stepping.");
    }

    #[test]
    fn test_distribution_error() {
        use rand::distr::uniform::Error as RandError;
        let rand_error = RandError::InvalidFloatBounds;
        let error = Error::Distribution(rand_error);
        assert!(error.to_string().contains("Distribution Error:"));
    }

    #[test]
    fn test_mj_init_error() {
        let error = Error::MjInitError("test error".to_string());
        assert_eq!(error.to_string(), "MuJoCo Initialization Error: test error");
    }

    #[test]
    fn test_invalid_action_dimension_error() {
        let error = Error::InvalidActionDimension { expected: 4, got: 3 };
        assert_eq!(error.to_string(), "Action dimension mismatch. Expected 4, got 3");
    }

    #[test]
    fn test_invalid_state_dimension_error() {
        let error = Error::InvalidStateDimension { 
            field: "position", 
            expected: 2, 
            got: 3 
        };
        assert_eq!(error.to_string(), "State dimension mismatch for position. Expected 2, got 3");
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = Error::InvalidMaxStep;
        
        // Test that Error implements the std::error::Error trait
        let std_error: &dyn StdError = &error;
        assert!(std_error.source().is_none());
    }

    #[test]
    fn test_error_derives() {
        let error1 = Error::InvalidAction;
        let error2 = Error::InvalidAction;
        
        // Test Debug implementation
        let debug_str = format!("{:?}", error1);
        assert!(debug_str.contains("InvalidAction"));
        
        // Test Clone implementation (through derive)
        let cloned_error = error1.clone();
        assert!(matches!(cloned_error, Error::InvalidAction));
    }

    #[test]
    fn test_error_from_trait_for_space_error() {
        // This tests that Error implements From<crate::spaces::Error>
        let space_error = SpaceError::InvalidSample;
        let error: Error = space_error.into();
        assert!(matches!(error, Error::Space(_)));
    }

    #[test]
    fn test_error_from_trait_for_rand_error() {
        use rand::distr::uniform::Error as RandError;
        
        // This tests that Error implements From<rand::distr::uniform::Error>
        let rand_error = RandError::InvalidFloatBounds;
        let error: Error = rand_error.into();
        assert!(matches!(error, Error::Distribution(_)));
    }
}