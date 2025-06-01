use super::Error;
use super::Experience;
use super::Terminal;

pub trait Environment {
    type Action;
    type State;
    type Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error>;
    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error>;
    fn is_terminal(&self) -> Result<bool, Error>;
    fn is_truncated(&self) -> bool;
    fn state(&self) -> Result<Self::State, Error>;
    #[inline]
    fn is_done(&self) -> Result<bool, Error> {
        Ok(self.is_terminal()? || self.is_truncated())
    }
    fn to_terminal(&self) -> Result<Terminal, Error> {
        Ok(Terminal::from_flags(self.is_terminal()?, self.is_truncated()))
    }
}
