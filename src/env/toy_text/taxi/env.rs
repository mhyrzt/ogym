use super::config::TaxiConfig;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use crate::spaces::{Discrete, EnvSpace, Space};
use rand::{rngs::StdRng, Rng, SeedableRng};

const SOUTH: u32 = 0;
const NORTH: u32 = 1;
const EAST: u32 = 2;
const WEST: u32 = 3;
const PICKUP: u32 = 4;
const DROPOFF: u32 = 5;

const GRID: usize = 5;
/// Pickup/dropoff locations R, G, Y, B as (row, col), matching the
/// classic Taxi-v3 map layout.
const LOCS: [(usize, usize); 4] = [(0, 0), (0, 4), (4, 0), (4, 3)];
/// Interior walls: (row, col) means a wall on the east side of that cell,
/// blocking movement between col and col+1 in that row.
const WALLS: [(usize, usize); 6] = [(0, 1), (1, 1), (3, 0), (3, 2), (4, 0), (4, 2)];

type Action = u32;
type State = u32;
type Info = ();

pub struct Taxi {
    config: TaxiConfig,
    row: usize,
    col: usize,
    pass_idx: usize, // 0..=3 waiting at LOCS[pass_idx], 4 = in taxi
    dest_idx: usize, // 0..=3
    steps: usize,
    rng: StdRng,
    pub space: EnvSpace<Discrete, Discrete>,
}

impl Taxi {
    pub fn new(config: TaxiConfig) -> Result<Self, Error> {
        let space = EnvSpace {
            state: Discrete::new(500)?,
            action: Discrete::new(6)?,
        };
        Ok(Self {
            config,
            row: 0,
            col: 0,
            pass_idx: 0,
            dest_idx: 1,
            steps: 0,
            rng: StdRng::from_os_rng(),
            space,
        })
    }

    fn encode(&self) -> u32 {
        (((self.row * GRID + self.col) * 5 + self.pass_idx) * 4 + self.dest_idx) as u32
    }

    fn wall_blocks_east(row: usize, col: usize) -> bool {
        WALLS.contains(&(row, col))
    }

    fn wall_blocks_west(row: usize, col: usize) -> bool {
        col == 0 || WALLS.contains(&(row, col - 1))
    }

    fn taxi_loc(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl Environment for Taxi {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        self.steps = 0;
        self.row = self.rng.random_range(0..GRID);
        self.col = self.rng.random_range(0..GRID);
        self.dest_idx = self.rng.random_range(0..4);
        // Passenger waits at one of the 4 locations, never already at the destination.
        loop {
            let candidate = self.rng.random_range(0..4);
            if candidate != self.dest_idx {
                self.pass_idx = candidate;
                break;
            }
        }
        Ok((self.encode(), ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.space
            .action
            .contains(&action)
            .map_err(|_| Error::InvalidAction)?;

        let curr_state = self.encode();
        let mut reward = -1.0;
        let mut terminated = false;

        match action {
            SOUTH => self.row = (self.row + 1).min(GRID - 1),
            NORTH => self.row = self.row.saturating_sub(1),
            EAST => {
                if !Self::wall_blocks_east(self.row, self.col) {
                    self.col = (self.col + 1).min(GRID - 1);
                }
            }
            WEST => {
                if !Self::wall_blocks_west(self.row, self.col) {
                    self.col = self.col.saturating_sub(1);
                }
            }
            PICKUP => {
                if self.pass_idx < 4 && self.taxi_loc() == LOCS[self.pass_idx] {
                    self.pass_idx = 4;
                } else {
                    reward = -10.0;
                }
            }
            DROPOFF => {
                if self.pass_idx == 4 && self.taxi_loc() == LOCS[self.dest_idx] {
                    self.pass_idx = self.dest_idx;
                    terminated = true;
                    reward = 20.0;
                } else if self.pass_idx == 4 {
                    if let Some(idx) = LOCS.iter().position(|&loc| loc == self.taxi_loc()) {
                        self.pass_idx = idx;
                    } else {
                        reward = -10.0;
                    }
                } else {
                    reward = -10.0;
                }
            }
            _ => {}
        }

        self.steps += 1;
        let next_state = self.encode();
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
        Ok(self.pass_idx == self.dest_idx
            && self.pass_idx < 4
            && self.taxi_loc() == LOCS[self.dest_idx])
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        Ok(self.encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_is_deterministic_with_seed() {
        let mut env = Taxi::new(TaxiConfig::default()).unwrap();
        let (s1, _) = env.reset(Some(7)).unwrap();
        let (s2, _) = env.reset(Some(7)).unwrap();
        assert_eq!(s1, s2);
        assert_ne!(env.pass_idx, env.dest_idx);
    }

    #[test]
    fn test_move_actions_stay_in_bounds() {
        let mut env = Taxi::new(TaxiConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        env.row = 0;
        env.col = 0;
        env.step(NORTH).unwrap();
        assert_eq!(env.row, 0);
        env.step(WEST).unwrap();
        assert_eq!(env.col, 0);
    }

    #[test]
    fn test_wall_blocks_movement_between_col1_and_col2_row0() {
        let mut env = Taxi::new(TaxiConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        env.row = 0;
        env.col = 1;
        env.step(EAST).unwrap();
        assert_eq!(env.col, 1, "wall at (0,1) should block eastward move");
    }

    #[test]
    fn test_pickup_and_dropoff_full_trip() {
        let mut env = Taxi::new(TaxiConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        env.pass_idx = 0; // waiting at R (0,0)
        env.dest_idx = 1; // going to G (0,4)
        env.row = 0;
        env.col = 0;

        let exp = env.step(PICKUP).unwrap();
        assert_eq!(env.pass_idx, 4);
        assert_eq!(exp.reward, -1.0);

        // illegal pickup while already carrying
        let exp = env.step(PICKUP).unwrap();
        assert_eq!(exp.reward, -10.0);

        env.row = 0;
        env.col = 4;
        let exp = env.step(DROPOFF).unwrap();
        assert_eq!(exp.reward, 20.0);
        assert!(exp.terminal.is_terminated());
        assert!(env.is_terminal().unwrap());
    }

    #[test]
    fn test_illegal_pickup_penalized() {
        let mut env = Taxi::new(TaxiConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        env.row = 2;
        env.col = 2;
        env.pass_idx = 0; // passenger at R (0,0), not here
        let exp = env.step(PICKUP).unwrap();
        assert_eq!(exp.reward, -10.0);
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = TaxiConfig {
            max_episode_steps: 3,
        };
        let mut env = Taxi::new(config).unwrap();
        env.reset(Some(1)).unwrap();
        for _ in 0..2 {
            let exp = env.step(SOUTH).unwrap();
            assert!(!exp.terminal.is_truncated());
        }
        let exp = env.step(SOUTH).unwrap();
        assert!(exp.terminal.is_truncated());
        assert!(env.is_truncated());
    }
}
