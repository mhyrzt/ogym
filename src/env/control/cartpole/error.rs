use crate::spaces::{continuous::ContinuousSpaceError, discrete::DiscreteSpaceError};
use rand::distr::uniform;

#[derive(Debug, thiserror::Error)]
pub enum CartPoleError {
    #[error("Invalid Max Step")]
    InvalidMaxStep,

    #[error("Invalid Action")]
    InvalidAction,

    #[error("Action error: {0}")]
    ActionError(#[from] DiscreteSpaceError),

    #[error("State error: {0}")]
    StateError(#[from] ContinuousSpaceError),

    #[error("Cannot take action: the environment is done. Call reset() to start a new episode.")]
    EpisodeDone,

    #[error("Environment has not been initialized. Call reset() before stepping.")]
    NotInitialized,

    #[error("Distribution Error: {0}")]
    DistError(#[from] uniform::Error),
}
