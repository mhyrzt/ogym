#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "ShapeMismatch: The `low` and `high` tensors must have the exact same shape. Ensure both inputs are aligned dimensionally."
    )]
    ShapeMismatch,

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
