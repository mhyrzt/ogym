pub mod env;
pub mod spaces;

#[cfg(test)]
mod tests {
    use crate::{
        env::control::cart_pole::{CartPole, CartPoleConfig},
        env::environment::Environment,
        spaces::Space,
    };

    #[test]
    fn cart_pole() {
        let config = CartPoleConfig::new();
        let mut env = CartPole::new(config).unwrap();
        env.reset(None).expect("Something Went Wrong");
        let mut total_reward: f64 = 0.0;
        loop {
            let action = env.space.action.sample().expect("Failed to sample Action");
            let experience = match env.step(action) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Step error: {:?}", e);
                    break;
                }
            };

            total_reward += experience.reward;

            if experience.terminal.is_done() {
                break;
            }
        }
        println!("Total Reward = {}", total_reward);
    }
}
