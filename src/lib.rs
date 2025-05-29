mod env;
mod spaces;

#[cfg(test)]
mod tests {
    use crate::{
        env::{
            Environment,
            control::cartpole::{CartPole, CartPoleConfig},
        },
        spaces::Space,
    };

    #[test]
    fn cart_pole() {
        let config = CartPoleConfig::builder();
        let mut env = CartPole::new(config).unwrap();
        let (mut state, _) = env.reset(None).expect("Something Went Wrong");
        println!("State: {}", state);
        let mut total_reward: f64 = 0.0;
        loop {
            let action = env.space.action.sample().expect("Failed to sample Action");
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
