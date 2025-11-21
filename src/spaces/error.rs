#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq)]
pub enum Error {
    #[error(
        "ShapeMismatch: The `low` and `high` tensors must have the exact same shape. Ensure both inputs are aligned dimensionally."
    )]
    ShapeMismatch,

    #[error("Space vector can not be empty")]
    EmptyVec,

    #[error("Type Mismatch")]
    TypeMismatch,

    #[error(
        "DiscreteInvalidSize: The `size` parameter must be greater than one. A size of one or less does not make sense in this context."
    )]
    DiscreteInvalidSize,

    #[error(
        "InvalidBounds: Every element in the `low` tensor must be less than or equal to its corresponding element in the `high` tensor. Check your range boundaries."
    )]
    InvalidBounds,

    #[error(
        "Distribution: Failed to create or sample from a uniform distribution. Underlying error: {0}"
    )]
    Distribution(#[from] rand::distr::uniform::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_implementation() {
        assert_eq!(
            Error::ShapeMismatch.to_string(),
            "ShapeMismatch: The `low` and `high` tensors must have the exact same shape. Ensure both inputs are aligned dimensionally."
        );

        assert_eq!(Error::EmptyVec.to_string(), "Space vector can not be empty");

        assert_eq!(Error::TypeMismatch.to_string(), "Type Mismatch");

        assert_eq!(
            Error::DiscreteInvalidSize.to_string(),
            "DiscreteInvalidSize: The `size` parameter must be greater than one. A size of one or less does not make sense in this context."
        );

        assert_eq!(
            Error::InvalidBounds.to_string(),
            "InvalidBounds: Every element in the `low` tensor must be less than or equal to its corresponding element in the `high` tensor. Check your range boundaries."
        );
    }

    #[test]
    fn test_error_traits() {
        fn assert_traits<
            T: std::error::Error + Send + Sync + Clone + Copy + PartialEq + std::fmt::Debug,
        >() {
        }
        assert_traits::<Error>();
    }

    #[test]
    fn test_clone_and_copy() {
        let err = Error::ShapeMismatch;
        let cloned = err;
        let copied = err;
        assert_eq!(err, cloned);
        assert_eq!(err, copied);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Error::ShapeMismatch, Error::ShapeMismatch);
        assert_ne!(Error::ShapeMismatch, Error::EmptyVec);
    }

    #[test]
    fn test_from_rand_error() {
        let rand_err = rand::distr::uniform::Error::EmptyRange;
        let error: Error = rand_err.into();

        if let Error::Distribution(inner) = error {
            assert_eq!(inner, rand::distr::uniform::Error::EmptyRange);
            let display = error.to_string();
            assert!(
                display.contains(
                    "Distribution: Failed to create or sample from a uniform distribution."
                ),
                "Missing prefix in: {}",
                display
            );
            // The actual message from rand's EmptyRange error
            assert!(
                display.contains("low > high") || display.contains("empty"),
                "Missing inner rand error message in: {}",
                display
            );
        } else {
            panic!("Expected Error::Distribution variant");
        }
    }
}
