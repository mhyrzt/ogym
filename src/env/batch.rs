use super::Error;

pub type BatchStepAllResult<S, I> = (Vec<S>, Vec<f64>, Vec<bool>, Vec<Option<I>>);

pub trait BatchEnvironment {
    type Action;
    type State;
    type Info;

    fn num_envs(&self) -> usize;

    fn reset_all(&mut self, seed: Option<u64>) -> Result<Vec<Self::State>, Error>;

    fn step_all(
        &mut self,
        actions: &[Self::Action],
    ) -> Result<BatchStepAllResult<Self::State, Self::Info>, Error>;

    fn reset_done(&mut self, dones: &[bool], seed: Option<u64>) -> Result<(), Error>;

    fn states(&self) -> Result<Vec<Self::State>, Error>;

    fn dones(&self) -> Result<Vec<bool>, Error>;
}
