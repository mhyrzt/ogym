use super::Error;

pub type StaticStepAllResult<S, I, const N: usize> = ([S; N], [f64; N], [bool; N], [Option<I>; N]);
pub trait StaticBatchEnvironment<const N: usize> {
    type Action;
    type State;
    type Info;

    type StepAllResult;

    fn reset_all(&mut self, seed: Option<u64>) -> Result<[Self::State; N], Error>;
    fn step_all(&mut self, actions: [Self::Action; N]) -> Result<Self::StepAllResult, Error>;
    fn reset_done(&mut self, dones: [bool; N], seed: Option<u64>) -> Result<(), Error>;
    fn states(&self) -> Result<[Self::State; N], Error>;
    fn dones(&self) -> Result<[bool; N], Error>;
}
