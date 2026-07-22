use super::config::BlackjackConfig;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use crate::spaces::{Discrete, EnvSpace, MultiDiscrete, Space};
use rand::{rngs::StdRng, Rng, SeedableRng};

const STICK: u32 = 0;
const HIT: u32 = 1;

/// Infinite deck: ace=1, 2-9 at face value, and four 10-valued entries
/// (10, J, Q, K all count as 10), matching Gymnasium's draw_card table.
const DECK: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

type Action = u32;
type State = Vec<u32>;
type Info = ();

pub struct Blackjack {
    config: BlackjackConfig,
    player: Vec<u8>,
    dealer: Vec<u8>,
    done: bool,
    rng: StdRng,
    pub space: EnvSpace<MultiDiscrete, Discrete>,
}

fn usable_ace(hand: &[u8]) -> bool {
    hand.contains(&1) && hand.iter().map(|&c| c as u32).sum::<u32>() + 10 <= 21
}

fn sum_hand(hand: &[u8]) -> u32 {
    let raw: u32 = hand.iter().map(|&c| c as u32).sum();
    if usable_ace(hand) {
        raw + 10
    } else {
        raw
    }
}

fn is_bust(hand: &[u8]) -> bool {
    sum_hand(hand) > 21
}

fn score(hand: &[u8]) -> u32 {
    if is_bust(hand) {
        0
    } else {
        sum_hand(hand)
    }
}

fn is_natural(hand: &[u8]) -> bool {
    hand.len() == 2 && ((hand[0] == 1 && hand[1] == 10) || (hand[0] == 10 && hand[1] == 1))
}

fn cmp(a: u32, b: u32) -> f64 {
    (a > b) as i32 as f64 - (a < b) as i32 as f64
}

impl Blackjack {
    pub fn new(config: BlackjackConfig) -> Result<Self, Error> {
        let space = EnvSpace {
            state: MultiDiscrete::new(vec![32, 11, 2])?,
            action: Discrete::new(2)?,
        };
        Ok(Self {
            config,
            player: Vec::new(),
            dealer: Vec::new(),
            done: false,
            rng: StdRng::from_os_rng(),
            space,
        })
    }

    fn draw_card(&mut self) -> u8 {
        DECK[self.rng.random_range(0..DECK.len())]
    }

    fn draw_hand(&mut self) -> Vec<u8> {
        vec![self.draw_card(), self.draw_card()]
    }

    fn obs(&self) -> State {
        vec![
            sum_hand(&self.player),
            self.dealer[0] as u32,
            usable_ace(&self.player) as u32,
        ]
    }
}

impl Environment for Blackjack {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        self.done = false;
        self.dealer = self.draw_hand();
        self.player = self.draw_hand();
        Ok((self.obs(), ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.space
            .action
            .contains(&action)
            .map_err(|_| Error::InvalidAction)?;

        let curr_state = self.obs();
        let mut reward;
        let terminated;

        match action {
            HIT => {
                let card = self.draw_card();
                self.player.push(card);
                if is_bust(&self.player) {
                    terminated = true;
                    reward = -1.0;
                } else {
                    terminated = false;
                    reward = 0.0;
                }
            }
            STICK => {
                while sum_hand(&self.dealer) < 17 {
                    let card = self.draw_card();
                    self.dealer.push(card);
                }
                terminated = true;
                reward = cmp(score(&self.player), score(&self.dealer));
                if self.config.natural && is_natural(&self.player) && reward == 1.0 {
                    reward = 1.5;
                }
            }
            _ => unreachable!("action space validated above"),
        }
        self.done = terminated;

        let next_state = self.obs();
        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            (),
            Terminal::from_flags(terminated, false),
            1,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(self.done)
    }

    fn is_truncated(&self) -> bool {
        false
    }

    fn state(&self) -> Result<Self::State, Error> {
        Ok(self.obs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_is_deterministic_with_seed() {
        let mut env = Blackjack::new(BlackjackConfig::default()).unwrap();
        let (s1, _) = env.reset(Some(3)).unwrap();
        let (s2, _) = env.reset(Some(3)).unwrap();
        assert_eq!(s1, s2);
        assert!(!env.done);
    }

    #[test]
    fn test_hit_adds_a_card_and_updates_sum() {
        let mut env = Blackjack::new(BlackjackConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        let before_len = env.player.len();
        let exp = env.step(HIT).unwrap();
        assert_eq!(env.player.len(), before_len + 1);
        assert_eq!(exp.next_state[0], sum_hand(&env.player));
    }

    #[test]
    fn test_hit_until_bust_terminates_with_negative_reward() {
        let mut env = Blackjack::new(BlackjackConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        env.player = vec![10, 9]; // sum 19, one more high card busts
        let mut exp = env.step(HIT).unwrap();
        while !exp.terminal.is_terminated() {
            exp = env.step(HIT).unwrap();
        }
        assert!(is_bust(&env.player));
        assert_eq!(exp.reward, -1.0);
        assert!(env.is_terminal().unwrap());
    }

    #[test]
    fn test_stick_plays_out_dealer_and_terminates() {
        let mut env = Blackjack::new(BlackjackConfig::default()).unwrap();
        env.reset(Some(1)).unwrap();
        let exp = env.step(STICK).unwrap();
        assert!(sum_hand(&env.dealer) >= 17 || is_bust(&env.dealer));
        assert!(exp.terminal.is_terminated());
        assert!([-1.0, 0.0, 1.0, 1.5].contains(&exp.reward));
    }

    #[test]
    fn test_usable_ace_and_sum_hand() {
        assert!(usable_ace(&[1, 6]));
        assert_eq!(sum_hand(&[1, 6]), 17);
        assert!(!usable_ace(&[1, 6, 10]));
        assert_eq!(sum_hand(&[1, 6, 10]), 17);
    }

    #[test]
    fn test_is_natural() {
        assert!(is_natural(&[1, 10]));
        assert!(is_natural(&[10, 1]));
        assert!(!is_natural(&[9, 2, 10]));
    }
}
