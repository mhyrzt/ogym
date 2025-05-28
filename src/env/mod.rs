pub mod control;

type StepResult<S, I> = (S, f64, bool, I);

pub trait Environment {
    type Action;
    type State;
    type Info;
    type Error;
    type ActionError;
    type StateError;
    type ActionSpace;
    type StateSpace;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Self::Error>;
    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<StepResult<Self::State, Self::Info>, Self::Error>;
    fn is_done(&self) -> Result<bool, Self::Error>;
    fn state(&self) -> Result<Self::State, Self::Error>;
}
