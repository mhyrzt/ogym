use ogym::env::environment::{Error, StaticBatchEnvironment};

struct MockStaticBatchEnvironment<const N: usize> {
    states: [i32; N],
    dones: [bool; N],
}

impl<const N: usize> MockStaticBatchEnvironment<N> {
    fn new() -> Self {
        Self {
            states: [0; N],
            dones: [false; N],
        }
    }
}

impl<const N: usize> StaticBatchEnvironment<N> for MockStaticBatchEnvironment<N> {
    type Action = i32;
    type State = i32;
    type Info = String;

    type StepAllResult = (
        [Self::State; N],
        [f64; N],
        [bool; N],
        [Option<Self::Info>; N],
    );

    fn reset_all(&mut self, _seed: Option<u64>) -> Result<[Self::State; N], Error> {
        self.states = [0; N];
        self.dones = [false; N];
        Ok(self.states)
    }

    fn step_all(&mut self, actions: [Self::Action; N]) -> Result<Self::StepAllResult, Error> {
        let mut new_states = [0; N];
        let mut rewards = [0.0; N];
        let mut new_dones = [false; N];
        let mut infos: [Option<Self::Info>; N] = [const { None }; N];

        for (i, &action) in actions.iter().enumerate() {
            let new_state = self.states[i] + action;
            new_states[i] = new_state;

            let reward = new_state as f64;
            rewards[i] = reward;

            let done = new_state >= 10; // arbitrary termination condition
            new_dones[i] = done;
            self.dones[i] = done;

            infos[i] = Some(format!("step_info_{}", i));

            self.states[i] = new_state;
        }

        Ok((new_states, rewards, new_dones, infos))
    }

    fn reset_done(&mut self, dones: [bool; N], _seed: Option<u64>) -> Result<(), Error> {
        for (i, &done) in dones.iter().enumerate() {
            if done {
                self.states[i] = 0;
                self.dones[i] = false;
            }
        }

        Ok(())
    }

    fn states(&self) -> Result<[Self::State; N], Error> {
        Ok(self.states)
    }

    fn dones(&self) -> Result<[bool; N], Error> {
        Ok(self.dones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_batch_environment_creation_2() {
        let env: MockStaticBatchEnvironment<2> = MockStaticBatchEnvironment::new();
        assert_eq!(env.states(), Ok([0, 0]));
    }

    #[test]
    fn test_static_batch_environment_creation_4() {
        let env: MockStaticBatchEnvironment<4> = MockStaticBatchEnvironment::new();
        assert_eq!(env.states(), Ok([0, 0, 0, 0]));
    }

    #[test]
    fn test_static_batch_reset_all() {
        let mut env: MockStaticBatchEnvironment<3> = MockStaticBatchEnvironment::new();

        env.states = [5, 10, 15];
        env.dones = [true, false, true];

        let states = env.reset_all(None).unwrap();
        assert_eq!(states, [0, 0, 0]);

        let current_states = env.states().unwrap();
        assert_eq!(current_states, [0, 0, 0]);

        let dones = env.dones().unwrap();
        assert_eq!(dones, [false, false, false]);
    }

    #[test]
    fn test_static_batch_step_all() {
        let mut env: MockStaticBatchEnvironment<2> = MockStaticBatchEnvironment::new();

        env.reset_all(None).unwrap();

        let actions = [1, 2];
        let (new_states, rewards, new_dones, infos) = env.step_all(actions).unwrap();

        assert_eq!(new_states, [1, 2]);
        assert_eq!(rewards, [1.0, 2.0]);
        assert_eq!(new_dones, [false, false]);
        assert_eq!(
            infos,
            [
                Some("step_info_0".to_string()),
                Some("step_info_1".to_string())
            ]
        );

        let current_states = env.states().unwrap();
        assert_eq!(current_states, [1, 2]);
    }

    #[test]
    fn test_static_batch_step_all_with_termination() {
        let mut env: MockStaticBatchEnvironment<2> = MockStaticBatchEnvironment::new();
        env.reset_all(None).unwrap();

        let actions = [15, 5];
        let (_, _, new_dones, _) = env.step_all(actions).unwrap();

        assert_eq!(new_dones, [true, false]);
    }

    #[test]
    fn test_static_batch_reset_done() {
        let mut env: MockStaticBatchEnvironment<3> = MockStaticBatchEnvironment::new();
        env.reset_all(None).unwrap();

        env.states = [10, 5, 15];
        env.dones = [true, false, true];

        env.reset_done([true, false, true], None).unwrap();

        let current_states = env.states().unwrap();
        assert_eq!(current_states, [0, 5, 0]); // only done envs should be reset

        let current_dones = env.dones().unwrap();
        assert_eq!(current_dones, [false, false, false]);
    }

    #[test]
    fn test_static_batch_states_and_dones() {
        let mut env: MockStaticBatchEnvironment<2> = MockStaticBatchEnvironment::new();
        env.reset_all(None).unwrap();

        env.step_all([3, 7]).unwrap();

        let states = env.states().unwrap();
        assert_eq!(states, [3, 7]);

        let dones = env.dones().unwrap();
        assert_eq!(dones, [false, false]);
    }

    #[test]
    fn test_static_batch_multiple_steps() {
        let mut env: MockStaticBatchEnvironment<2> = MockStaticBatchEnvironment::new();
        env.reset_all(None).unwrap();

        let (states, _, dones, _) = env.step_all([1, 1]).unwrap();
        assert_eq!(states, [1, 1]);
        assert_eq!(dones, [false, false]);

        let (states, _, dones, _) = env.step_all([2, 3]).unwrap();
        assert_eq!(states, [3, 4]);
        assert_eq!(dones, [false, false]);

        let (states, _, dones, _) = env.step_all([8, 7]).unwrap();
        assert_eq!(states, [11, 11]);
        assert_eq!(dones, [true, true]); // Both should be done now
    }
}
