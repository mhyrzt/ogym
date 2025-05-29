use super::Error;

pub type StaticStepAllResult<S, I, const N: usize> = ([S; N], [f64; N], [bool; N], [Option<I>; N]);
pub trait StaticBatchEnvironment<const N: usize> {
    type Action;
    type State;
    type Info;

    /// The result type for stepping all environments.
    type StepAllResult;

    /// Reset all environments.
    fn reset_all(&mut self, seed: Option<u64>) -> Result<[Self::State; N], Error>;

    /// Step all environments with the given actions.
    fn step_all(&mut self, actions: [Self::Action; N]) -> Result<Self::StepAllResult, Error>;

    /// Optionally reset only completed environments (using mask).
    fn reset_done(&mut self, dones: [bool; N], seed: Option<u64>) -> Result<(), Error>;

    /// Get current states.
    fn states(&self) -> Result<[Self::State; N], Error>;

    /// Get done flags.
    fn dones(&self) -> Result<[bool; N], Error>;
}
