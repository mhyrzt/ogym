mod env;
mod spaces;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{
        env::{Environment, control::cartpole::env::CartPole},
        spaces::space::Space,
    };

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cart_pole() {
        let mut env = CartPole::new().unwrap();
        let (mut state, _) = env.reset(None).expect("Something Went Wrong");
        println!("State: {}", state);
        let mut total_reward: f64 = 0.0;
        loop {
            let action = env.space.action.sample().unwrap();
            let (next_state, reward, done, _) = match env.step(action) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Step error: {:?}", e);
                    break;
                }
            };

            state = next_state;
            total_reward += reward;

            if done {
                break;
            }
        }
        println!("State: {}", state);
        println!("Total Reward = {}", total_reward);
    }
}
