use super::Error;

pub type StepResult<S, I> = (S, f64, bool, I);

pub trait Environment {
    type Action;
    type State;
    type Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error>;
    fn step(&mut self, action: Self::Action) -> Result<StepResult<Self::State, Self::Info>, Error>;
    fn is_done(&self) -> Result<bool, Error>;
    fn state(&self) -> Result<Self::State, Error>;
}