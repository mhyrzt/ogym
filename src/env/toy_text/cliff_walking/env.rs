use super::config::CliffWalkingConfig;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use crate::spaces::{Discrete, EnvSpace, Space};

const UP: u32 = 0;
const RIGHT: u32 = 1;
const DOWN: u32 = 2;
const LEFT: u32 = 3;

type Action = u32;
type State = u32;
type Info = ();

pub struct CliffWalking {
    config: CliffWalkingConfig,
    state: u32,
    steps: usize,
    pub space: EnvSpace<Discrete, Discrete>,
}

impl CliffWalking {
    pub fn new(config: CliffWalkingConfig) -> Result<Self, Error> {
        let space = EnvSpace {
            state: Discrete::new((config.nrow * config.ncol) as u32)?,
            action: Discrete::new(4)?,
        };
        let start = Self::start_state(&config);
        Ok(Self {
            config,
            state: start,
            steps: 0,
            space,
        })
    }

    fn start_state(config: &CliffWalkingConfig) -> u32 {
        ((config.nrow - 1) * config.ncol) as u32
    }

    fn goal_state(&self) -> u32 {
        (self.config.nrow * self.config.ncol - 1) as u32
    }

    fn is_cliff(&self, row: usize, col: usize) -> bool {
        row == self.config.nrow - 1 && col > 0 && col < self.config.ncol - 1
    }

    /// Matches Gymnasium: moving into the cliff strip snaps back to the
    /// start with a -100 penalty but does NOT terminate the episode.
    fn transition(&self, action: u32) -> (u32, f64, bool) {
        let row = self.state as usize / self.config.ncol;
        let col = self.state as usize % self.config.ncol;
        let (dr, dc): (isize, isize) = match action {
            UP => (-1, 0),
            RIGHT => (0, 1),
            DOWN => (1, 0),
            LEFT => (0, -1),
            _ => (0, 0),
        };
        let new_row = (row as isize + dr).clamp(0, self.config.nrow as isize - 1) as usize;
        let new_col = (col as isize + dc).clamp(0, self.config.ncol as isize - 1) as usize;

        if self.is_cliff(new_row, new_col) {
            return (Self::start_state(&self.config), -100.0, false);
        }
        let new_state = (new_row * self.config.ncol + new_col) as u32;
        let terminated = new_state == self.goal_state();
        (new_state, -1.0, terminated)
    }
}

impl Environment for CliffWalking {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, _seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.steps = 0;
        self.state = Self::start_state(&self.config);
        Ok((self.state, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.space
            .action
            .contains(&action)
            .map_err(|_| Error::InvalidAction)?;

        let curr_state = self.state;
        let (next_state, reward, terminated) = self.transition(action);
        self.state = next_state;
        self.steps += 1;
        let truncated = self.steps >= self.config.max_episode_steps;

        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            (),
            Terminal::from_flags(terminated, truncated),
            self.steps as u32,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(self.state == self.goal_state())
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        Ok(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_is_deterministic_at_bottom_left() {
        let mut env = CliffWalking::new(CliffWalkingConfig::default()).unwrap();
        let (s1, _) = env.reset(Some(1)).unwrap();
        let (s2, _) = env.reset(Some(99)).unwrap();
        assert_eq!(s1, 36); // row3,col0 of a 4x12 grid
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_step_up_then_right_moves_off_cliff_row() {
        let mut env = CliffWalking::new(CliffWalkingConfig::default()).unwrap();
        env.reset(None).unwrap();
        let exp = env.step(UP).unwrap();
        assert_eq!(exp.next_state, 24); // row2,col0
        assert_eq!(exp.reward, -1.0);
        assert!(!exp.terminal.is_done());
    }

    #[test]
    fn test_stepping_into_cliff_resets_to_start_with_penalty() {
        let mut env = CliffWalking::new(CliffWalkingConfig::default()).unwrap();
        env.reset(None).unwrap();
        let exp = env.step(RIGHT).unwrap(); // row3,col1 is cliff
        assert_eq!(exp.reward, -100.0);
        assert_eq!(exp.next_state, 36); // snapped back to start
        assert!(!exp.terminal.is_terminated());
    }

    #[test]
    fn test_reaching_goal_terminates() {
        let mut env = CliffWalking::new(CliffWalkingConfig::default()).unwrap();
        env.reset(None).unwrap();
        env.step(UP).unwrap();
        for _ in 0..11 {
            env.step(RIGHT).unwrap();
        }
        let exp = env.step(DOWN).unwrap();
        assert_eq!(exp.next_state, env.goal_state());
        assert!(exp.terminal.is_terminated());
        assert_eq!(exp.reward, -1.0);
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = CliffWalkingConfig {
            max_episode_steps: 3,
            ..CliffWalkingConfig::default()
        };
        let mut env = CliffWalking::new(config).unwrap();
        env.reset(None).unwrap();
        for _ in 0..2 {
            let exp = env.step(UP).unwrap();
            assert!(!exp.terminal.is_truncated());
        }
        let exp = env.step(DOWN).unwrap();
        assert!(exp.terminal.is_truncated());
        assert!(env.is_truncated());
    }
}
