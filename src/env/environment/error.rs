#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum Error {
    #[error("Invalid Max Step")]
    InvalidMaxStep,

    #[error("Invalid Action")]
    InvalidAction,

    #[error("Space error: {0}")]
    Space(#[from] crate::spaces::Error),

    #[error("Cannot take action: the environment is done. Call reset() to start a new episode.")]
    EpisodeDone,

    #[error("Environment has not been initialized. Call reset() before stepping.")]
    NotInitialized,

    #[error("Distribution Error: {0}")]
    Distribution(#[from] rand::distr::uniform::Error),

    #[error("MuJoCo Initialization Error: {0}")]
    MjInitError(String),

    #[error("Action dimension mismatch. Expected {expected}, got {got}")]
    InvalidActionDimension { expected: usize, got: usize },

    #[error("State dimension mismatch for {field}. Expected {expected}, got {got}")]
    InvalidStateDimension {
        field: &'static str,
        expected: usize,
        got: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_simple_variants() {
        assert_eq!(Error::InvalidMaxStep.to_string(), "Invalid Max Step");
        assert_eq!(Error::InvalidAction.to_string(), "Invalid Action");
        assert_eq!(
            Error::EpisodeDone.to_string(),
            "Cannot take action: the environment is done. Call reset() to start a new episode."
        );
        assert_eq!(
            Error::NotInitialized.to_string(),
            "Environment has not been initialized. Call reset() before stepping."
        );
    }

    #[test]
    fn test_error_messages_with_fields() {
        let err = Error::MjInitError("Failed to load model".to_string());
        assert_eq!(
            err.to_string(),
            "MuJoCo Initialization Error: Failed to load model"
        );

        let err = Error::InvalidActionDimension {
            expected: 4,
            got: 2,
        };
        assert_eq!(
            err.to_string(),
            "Action dimension mismatch. Expected 4, got 2"
        );

        let err = Error::InvalidStateDimension {
            field: "observation",
            expected: 10,
            got: 5,
        };
        assert_eq!(
            err.to_string(),
            "State dimension mismatch for observation. Expected 10, got 5"
        );
    }

    #[test]
    fn test_clone_and_partial_eq() {
        let err1 = Error::InvalidMaxStep;
        let err2 = err1.clone();
        assert_eq!(err1, err2);

        let err3 = Error::InvalidActionDimension {
            expected: 3,
            got: 1,
        };
        let err4 = err3.clone();
        assert_eq!(err3, err4);
        assert_ne!(err1, err3);
    }
}
