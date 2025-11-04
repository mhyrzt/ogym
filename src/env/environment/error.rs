#[derive(Debug, thiserror::Error)]
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
}
