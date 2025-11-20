use ogym::env::environment::{Environment, Experience, Error, Terminal};

// Mock implementation of Environment for testing
struct MockEnvironment {
    state: i32,
    done: bool,
    terminated: bool,
    truncated: bool,
}

impl MockEnvironment {
    fn new() -> Self {
        Self {
            state: 0,
            done: false,
            terminated: false,
            truncated: false,
        }
    }
}

impl Environment for MockEnvironment {
    type Action = i32;
    type State = i32;
    type Info = String;

    fn reset(&mut self, _seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.state = 0;
        self.done = false;
        self.terminated = false;
        self.truncated = false;
        Ok((self.state, "reset_info".to_string()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        // Store original termination status to preserve manual settings
        let was_terminated = self.terminated;
        let was_truncated = self.truncated;

        // Apply action to get new state
        let next_state = self.state + action;

        // Calculate reward
        let reward = next_state as f64;

        // Update terminated based on the new state, but preserve existing termination
        self.terminated = self.terminated || next_state >= 10;  // arbitrary termination condition
        // truncated is preserved as it's not reset in the step logic

        self.state = next_state;
        self.done = self.terminated || self.truncated;

        let terminal = Terminal::from_flags(self.terminated, self.truncated);

        Ok(Experience::new(
            next_state - action, // current state (before action)
            reward,
            action,
            next_state, // next state
            "step_info".to_string(), // info
            terminal,
            0, // step count
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_reset() {
        let mut env = MockEnvironment::new();
        
        // Perform an action first to change state
        env.state = 5;
        env.done = true;
        
        let (state, info) = env.reset(None).unwrap();
        assert_eq!(state, 0);
        assert_eq!(info, "reset_info");
        assert_eq!(env.state, 0);
        assert_eq!(env.done, false);
    }

    #[test]
    fn test_environment_step() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        let experience = env.step(3).unwrap();
        
        // Check the experience fields
        assert_eq!(experience.curr_state, 0);  // state before action
        assert_eq!(experience.action, 3);
        assert_eq!(experience.reward, 3.0);    // new state as reward
        assert_eq!(experience.next_state, 3);  // state after action
        assert_eq!(experience.info, "step_info");
        assert_eq!(experience.terminal, Terminal::Ongoing);
        assert_eq!(experience.step, 0);
        
        // Check internal state
        assert_eq!(env.state, 3);
    }

    #[test]
    fn test_environment_termination() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Take an action that causes termination
        let experience = env.step(15).unwrap();
        
        assert_eq!(experience.terminal, Terminal::Terminate);
        assert_eq!(env.is_terminal().unwrap(), true);
        assert_eq!(env.is_done().unwrap(), true);
    }

    #[test]
    fn test_environment_truncation() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Manually set truncation flag
        env.truncated = true;
        env.terminated = false;  // Not terminated, but truncated
        
        // Take a step to trigger the truncation
        let experience = env.step(2).unwrap();
        
        assert_eq!(experience.terminal, Terminal::Truncate);
        assert_eq!(env.is_truncated(), true);
        assert_eq!(env.is_done().unwrap(), true);
        assert_eq!(env.is_terminal().unwrap(), false);
    }

    #[test]
    fn test_environment_both_terminate_and_truncate() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Manually set both flags
        env.truncated = true;
        env.terminated = true;
        
        let experience = env.step(2).unwrap();
        
        assert_eq!(experience.terminal, Terminal::Both);
        assert_eq!(env.is_truncated(), true);
        assert_eq!(env.is_terminal().unwrap(), true);
        assert_eq!(env.is_done().unwrap(), true);
    }

    #[test]
    fn test_environment_state_access() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Perform some actions to change state
        env.step(5).unwrap();
        env.step(3).unwrap();
        
        let current_state = env.state().unwrap();
        assert_eq!(current_state, 8);
    }

    #[test]
    fn test_is_done() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Initially not done
        assert_eq!(env.is_done().unwrap(), false);
        
        // Set termination condition
        env.terminated = true;
        assert_eq!(env.is_done().unwrap(), true);
        
        // Reset termination, set truncation
        env.terminated = false;
        env.truncated = true;
        assert_eq!(env.is_done().unwrap(), true);
        
        // Both false
        env.truncated = false;
        assert_eq!(env.is_done().unwrap(), false);
    }

    #[test]
    fn test_to_terminal() {
        let mut env = MockEnvironment::new();
        env.reset(None).unwrap();
        
        // Not done
        env.terminated = false;
        env.truncated = false;
        assert_eq!(env.to_terminal().unwrap(), Terminal::Ongoing);
        
        // Terminated only
        env.terminated = true;
        env.truncated = false;
        assert_eq!(env.to_terminal().unwrap(), Terminal::Terminate);
        
        // Truncated only
        env.terminated = false;
        env.truncated = true;
        assert_eq!(env.to_terminal().unwrap(), Terminal::Truncate);
        
        // Both
        env.terminated = true;
        env.truncated = true;
        assert_eq!(env.to_terminal().unwrap(), Terminal::Both);
    }
}