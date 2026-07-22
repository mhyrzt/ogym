use super::config::FrozenLakeConfig;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use crate::spaces::{Discrete, EnvSpace, Space};
use rand::{rngs::StdRng, Rng, SeedableRng};

const LEFT: u32 = 0;
const DOWN: u32 = 1;
const RIGHT: u32 = 2;
const UP: u32 = 3;

type Action = u32;
type State = u32;
type Info = ();

pub struct FrozenLake {
    config: FrozenLakeConfig,
    nrow: usize,
    ncol: usize,
    state: u32,
    steps: usize,
    rng: StdRng,
    pub space: EnvSpace<Discrete, Discrete>,
}

impl FrozenLake {
    pub fn new(config: FrozenLakeConfig) -> Result<Self, Error> {
        let nrow = config.map.len();
        let ncol = config.map[0].len();
        let space = EnvSpace {
            state: Discrete::new((nrow * ncol) as u32)?,
            action: Discrete::new(4)?,
        };
        let start = Self::find_start(&config.map, ncol);
        Ok(Self {
            config,
            nrow,
            ncol,
            state: start,
            steps: 0,
            rng: StdRng::from_os_rng(),
            space,
        })
    }

    fn find_start(map: &[Vec<u8>], ncol: usize) -> u32 {
        for (r, row) in map.iter().enumerate() {
            for (c, &tile) in row.iter().enumerate() {
                if tile == b'S' {
                    return (r * ncol + c) as u32;
                }
            }
        }
        0
    }

    fn tile(&self, s: u32) -> u8 {
        self.config.map[s as usize / self.ncol][s as usize % self.ncol]
    }

    /// Deterministic single-step move used both by the real dynamics and,
    /// under `is_slippery`, by each of the three candidate directions.
    fn inc(&self, s: u32, action: u32) -> u32 {
        let row = s as usize / self.ncol;
        let col = s as usize % self.ncol;
        let (row, col) = match action {
            LEFT => (row, col.saturating_sub(1)),
            DOWN => ((row + 1).min(self.nrow - 1), col),
            RIGHT => (row, (col + 1).min(self.ncol - 1)),
            UP => (row.saturating_sub(1), col),
            _ => (row, col),
        };
        (row * self.ncol + col) as u32
    }

    /// Matches Gymnasium: under is_slippery, the actual move is chosen
    /// uniformly among {action-1, action, action+1} (mod 4), each 1/3.
    fn resolve_move(&mut self, action: u32) -> u32 {
        if self.config.is_slippery {
            let candidates = [(action + 3) % 4, action, (action + 1) % 4];
            let pick = candidates[self.rng.random_range(0..3)];
            self.inc(self.state, pick)
        } else {
            self.inc(self.state, action)
        }
    }
}

impl Environment for FrozenLake {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        self.steps = 0;
        self.state = Self::find_start(&self.config.map, self.ncol);
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
        let next_state = self.resolve_move(action);
        self.state = next_state;
        self.steps += 1;

        let letter = self.tile(next_state);
        let terminated = letter == b'H' || letter == b'G';
        let reward = if letter == b'G' { 1.0 } else { 0.0 };
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
        let letter = self.tile(self.state);
        Ok(letter == b'H' || letter == b'G')
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

    fn non_slippery_config() -> FrozenLakeConfig {
        FrozenLakeConfig {
            is_slippery: false,
            ..FrozenLakeConfig::default()
        }
    }

    #[test]
    fn test_reset_is_deterministic_at_start_tile() {
        let mut env = FrozenLake::new(FrozenLakeConfig::default()).unwrap();
        let (s1, _) = env.reset(Some(1)).unwrap();
        let (s2, _) = env.reset(Some(2)).unwrap();
        assert_eq!(s1, 0);
        assert_eq!(s2, 0);
        assert_eq!(env.tile(s1), b'S');
    }

    #[test]
    fn test_deterministic_step_moves_right_and_down() {
        let mut env = FrozenLake::new(non_slippery_config()).unwrap();
        env.reset(Some(0)).unwrap();

        let exp = env.step(RIGHT).unwrap();
        assert_eq!(exp.next_state, 1);
        assert!(!exp.terminal.is_done());

        let exp = env.step(DOWN).unwrap();
        assert_eq!(exp.next_state, 5); // row1,col1 -> 'H'
        assert!(exp.terminal.is_terminated());
        assert_eq!(exp.reward, 0.0);
    }

    #[test]
    fn test_reaching_goal_terminates_with_reward_one() {
        let mut env = FrozenLake::new(non_slippery_config()).unwrap();
        env.reset(Some(0)).unwrap();
        // Default map: SFFF/FHFH/FFFH/HFFG - this path skirts both holes.
        for action in [DOWN, DOWN, RIGHT, RIGHT, DOWN, RIGHT] {
            let exp = env.step(action).unwrap();
            if exp.terminal.is_terminated() {
                assert_eq!(exp.reward, 1.0);
                assert_eq!(env.tile(exp.next_state), b'G');
                return;
            }
        }
        panic!("expected to reach the goal");
    }

    #[test]
    fn test_bump_into_wall_stays_in_place() {
        let mut env = FrozenLake::new(non_slippery_config()).unwrap();
        env.reset(Some(0)).unwrap();
        let exp = env.step(UP).unwrap();
        assert_eq!(exp.next_state, 0);
        let exp = env.step(LEFT).unwrap();
        assert_eq!(exp.next_state, 0);
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = FrozenLakeConfig {
            is_slippery: false,
            max_episode_steps: 3,
            ..FrozenLakeConfig::default()
        };
        let mut env = FrozenLake::new(config).unwrap();
        env.reset(Some(0)).unwrap();
        // LEFT is a no-op on the start tile, so this never terminates early.
        for _ in 0..2 {
            let exp = env.step(LEFT).unwrap();
            assert!(!exp.terminal.is_truncated());
        }
        let exp = env.step(LEFT).unwrap();
        assert!(exp.terminal.is_truncated());
        assert!(env.is_truncated());
    }

    #[test]
    fn test_invalid_action_rejected() {
        let mut env = FrozenLake::new(FrozenLakeConfig::default()).unwrap();
        env.reset(Some(0)).unwrap();
        assert_eq!(env.step(4), Err(Error::InvalidAction));
    }
}
