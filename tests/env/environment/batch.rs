use ogym::env::environment::{BatchEnvironment, Error};

// Mock implementation of BatchEnvironment for testing
struct MockBatchEnvironment {
    num_envs: usize,
    states: Vec<i32>,
    dones: Vec<bool>,
}

impl MockBatchEnvironment {
    fn new(num_envs: usize) -> Self {
        Self {
            num_envs,
            states: vec![0; num_envs],
            dones: vec![false; num_envs],
        }
    }
}

impl BatchEnvironment for MockBatchEnvironment {
    type Action = i32;
    type State = i32;
    type Info = String;

    fn num_envs(&self) -> usize {
        self.num_envs
    }

    fn reset_all(&mut self, _seed: Option<u64>) -> Result<Vec<Self::State>, Error> {
        self.states = vec![0; self.num_envs];
        self.dones = vec![false; self.num_envs];
        Ok(self.states.clone())
    }

    fn step_all(
        &mut self,
        actions: &[Self::Action],
    ) -> Result<(Vec<Self::State>, Vec<f64>, Vec<bool>, Vec<Option<Self::Info>>), Error> {
        if actions.len() != self.num_envs {
            return Err(Error::InvalidActionDimension {
                expected: self.num_envs,
                got: actions.len(),
            });
        }

        let mut new_states = Vec::new();
        let mut rewards = Vec::new();
        let mut new_dones = Vec::new();
        let mut infos = Vec::new();

        for (i, &action) in actions.iter().enumerate() {
            // Apply action to get new state
            let new_state = self.states[i] + action;
            new_states.push(new_state);

            // Calculate reward (simple function of state)
            let reward = new_state as f64;
            rewards.push(reward);

            // Determine if environment is done based on state
            let done = new_state >= 10; // arbitrary termination condition
            new_dones.push(done);
            self.dones[i] = done;

            // Create info
            let info = Some(format!("step_info_{}", i));
            infos.push(info);

            // Update internal state
            self.states[i] = new_state;
        }

        Ok((new_states, rewards, new_dones, infos))
    }

    fn reset_done(&mut self, dones: &[bool], _seed: Option<u64>) -> Result<(), Error> {
        if dones.len() != self.num_envs {
            return Err(Error::InvalidActionDimension {
                expected: self.num_envs,
                got: dones.len(),
            });
        }

        for (i, &done) in dones.iter().enumerate() {
            if done {
                self.states[i] = 0;
                self.dones[i] = false;
            }
        }

        Ok(())
    }

    fn states(&self) -> Result<Vec<Self::State>, Error> {
        Ok(self.states.clone())
    }

    fn dones(&self) -> Result<Vec<bool>, Error> {
        Ok(self.dones.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_environment_creation() {
        let env = MockBatchEnvironment::new(3);
        assert_eq!(env.num_envs(), 3);
    }

    #[test]
    fn test_reset_all() {
        let mut env = MockBatchEnvironment::new(4);
        
        // Initialize with non-zero values to test reset
        env.states = vec![5, 10, 15, 20];
        env.dones = vec![true, false, true, false];
        
        let states = env.reset_all(None).unwrap();
        assert_eq!(states.len(), 4);
        assert_eq!(states, vec![0, 0, 0, 0]);
        
        let current_states = env.states().unwrap();
        assert_eq!(current_states, vec![0, 0, 0, 0]);
        
        let dones = env.dones().unwrap();
        assert_eq!(dones, vec![false, false, false, false]);
    }

    #[test]
    fn test_step_all() {
        let mut env = MockBatchEnvironment::new(2);
        
        // Reset to initial state
        env.reset_all(None).unwrap();
        
        // Step with actions
        let actions = vec![1, 2];
        let (new_states, rewards, new_dones, infos) = env.step_all(&actions).unwrap();
        
        // Check that states were updated correctly
        assert_eq!(new_states, vec![1, 2]);
        assert_eq!(rewards, vec![1.0, 2.0]);
        assert_eq!(new_dones, vec![false, false]);
        assert_eq!(infos, vec![Some("step_info_0".to_string()), Some("step_info_1".to_string())]);
        
        // Check that internal state was updated
        let current_states = env.states().unwrap();
        assert_eq!(current_states, vec![1, 2]);
    }

    #[test]
    fn test_step_all_with_termination() {
        let mut env = MockBatchEnvironment::new(2);
        env.reset_all(None).unwrap();
        
        // Step with action that causes termination
        let actions = vec![15, 5]; // first action will cause state to exceed 10
        let (_, _, new_dones, _) = env.step_all(&actions).unwrap();
        
        assert_eq!(new_dones, vec![true, false]);
    }

    #[test]
    fn test_reset_done() {
        let mut env = MockBatchEnvironment::new(3);
        env.reset_all(None).unwrap();
        
        // Manually set some states and dones to test reset_done
        env.states = vec![10, 5, 15];
        env.dones = vec![true, false, true];
        
        // Reset only the done environments
        env.reset_done(&vec![true, false, true], None).unwrap();
        
        let current_states = env.states().unwrap();
        assert_eq!(current_states, vec![0, 5, 0]); // only done envs should be reset
        
        let current_dones = env.dones().unwrap();
        assert_eq!(current_dones, vec![false, false, false]);
    }

    #[test]
    fn test_invalid_action_dimension() {
        let mut env = MockBatchEnvironment::new(3);
        let result = env.step_all(&vec![1, 2]); // Wrong number of actions
        
        assert!(matches!(result, Err(Error::InvalidActionDimension { .. })));
    }

    #[test]
    fn test_states_and_dones() {
        let mut env = MockBatchEnvironment::new(2);
        env.reset_all(None).unwrap();

        // Step to change state
        env.step_all(&vec![3, 7]).unwrap();

        let states = env.states().unwrap();
        assert_eq!(states, vec![3, 7]);

        let dones = env.dones().unwrap();
        assert_eq!(dones, vec![false, false]);
    }
}