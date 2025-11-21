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
        Ok(Terminal::from_flags(
            self.is_terminal()?,
            self.is_truncated(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::environment::terminal::Terminal;

    struct MockEnv {
        terminated: bool,
        truncated: bool,
        state: i32,
    }

    impl MockEnv {
        fn new(terminated: bool, truncated: bool) -> Self {
            Self {
                terminated,
                truncated,
                state: 0,
            }
        }
    }

    impl Environment for MockEnv {
        type Action = i32;
        type State = i32;
        type Info = ();

        fn reset(&mut self, _seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
            self.state = 0;
            Ok((self.state, ()))
        }

        fn step(
            &mut self,
            action: Self::Action,
        ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
            self.state += action;
            Ok(Experience::new(
                self.state - action,
                1.0,
                action,
                self.state,
                (),
                Terminal::Ongoing,
                1,
            ))
        }

        fn is_terminal(&self) -> Result<bool, Error> {
            Ok(self.terminated)
        }

        fn is_truncated(&self) -> bool {
            self.truncated
        }

        fn state(&self) -> Result<Self::State, Error> {
            Ok(self.state)
        }
    }

    #[test]
    fn test_is_done_default_impl() {
        let env = MockEnv::new(false, false);
        assert_eq!(env.is_done(), Ok(false));

        let env = MockEnv::new(true, false);
        assert_eq!(env.is_done(), Ok(true));

        let env = MockEnv::new(false, true);
        assert_eq!(env.is_done(), Ok(true));

        let env = MockEnv::new(true, true);
        assert_eq!(env.is_done(), Ok(true));
    }

    #[test]
    fn test_to_terminal_default_impl() {
        let env = MockEnv::new(false, false);
        assert_eq!(env.to_terminal(), Ok(Terminal::Ongoing));

        let env = MockEnv::new(true, false);
        assert_eq!(env.to_terminal(), Ok(Terminal::Terminate));

        let env = MockEnv::new(false, true);
        assert_eq!(env.to_terminal(), Ok(Terminal::Truncate));

        let env = MockEnv::new(true, true);
        assert_eq!(env.to_terminal(), Ok(Terminal::Both));
    }

    #[test]
    fn test_mock_interaction() {
        let mut env = MockEnv::new(false, false);

        let reset_res = env.reset(None);
        assert!(reset_res.is_ok());
        assert_eq!(reset_res.unwrap(), (0, ()));

        let step_res = env.step(5);
        assert!(step_res.is_ok());
        let exp = step_res.unwrap();
        assert_eq!(exp.curr_state, 0);
        assert_eq!(exp.next_state, 5);
        assert_eq!(exp.action, 5);

        assert_eq!(env.state(), Ok(5));
    }
}
