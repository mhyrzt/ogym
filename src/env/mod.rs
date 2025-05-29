pub mod control;

type StepResult<S, I> = (S, f64, bool, I);

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
}

pub trait Environment {
    type Action;
    type State;
    type Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error>;
    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<StepResult<Self::State, Self::Info>, Error>;
    fn is_done(&self) -> Result<bool, Error>;
    fn state(&self) -> Result<Self::State, Error>;
}
