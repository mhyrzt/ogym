use super::config::SwimmerConfig;
use crate::env::environment::{Environment, Experience, Terminal};
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoSwimmerEnv {
    pub env: MjEnv,
    pub config: SwimmerConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
    steps: usize,
}

impl MujocoSwimmerEnv {
    pub fn new(config: Option<SwimmerConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(&config.xml_file, config.frame_skip)?;
        Ok(Self {
            init_qpos: env.init_qpos().into(),
            init_qvel: env.init_qvel().into(),
            config,
            env,
            steps: 0,
        })
    }

    // Helper methods
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        let mut position = self.env.qpos().to_vec();
        let velocity = self.env.qvel().to_vec();

        if self.config.exclude_current_positions_from_observation {
            // Skip first 2 elements (x, y position)
            position = position[2..].to_vec();
        }

        observation.extend_from_slice(&position);
        observation.extend_from_slice(&velocity);

        Ok(DVector::from_vec(observation))
    }

    fn _calculate_reward(&self, x_velocity: f64, action: &DVector<f64>) -> Result<f64, Error> {
        let forward_reward = self.config.forward_reward_weight * x_velocity;
        let ctrl_cost = self._control_cost(action)?;

        Ok(forward_reward - ctrl_cost)
    }

    fn _control_cost(&self, action: &DVector<f64>) -> Result<f64, Error> {
        let squared_actions: f64 = action.iter().map(|x| x * x).sum();
        Ok(self.config.ctrl_cost_weight * squared_actions)
    }

    fn _get_reset_info(&self) -> Result<HashMap<String, f64>, Error> {
        let mut info = HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        Ok(info)
    }
}

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoSwimmerEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.steps = 0;
        self.env.reset_to_initial()?;

        // Apply noise to initial state
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };

        let noise_low = -self.config.reset_noise_scale;
        let noise_high = self.config.reset_noise_scale;

        // Add noise to positions
        let mut qpos = self.init_qpos.clone();
        if noise_high > noise_low {
            for i in 0..qpos.len() {
                qpos[i] += rng.random_range(noise_low..noise_high);
            }
        }

        // Add noise to velocities
        let mut qvel = self.init_qvel.clone();
        if noise_high > noise_low {
            for i in 0..qvel.len() {
                qvel[i] += rng.random_range(noise_low..noise_high);
            }
        }

        self.env.set_state(&qpos, &qvel)?;

        let observation = self._get_obs()?;
        let info = self._get_reset_info()?;

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        // Store current state
        let curr_state = self.state()?;

        // Get position before simulation
        let x_position_before = self.env.qpos()[0];

        // Apply action and simulate
        self.env.do_simulation(action.as_slice())?;
        self.steps += 1;

        // Get position after simulation
        let x_position_after = self.env.qpos()[0];

        // Calculate velocity
        let dt = self.env.dt();
        let x_velocity = (x_position_after - x_position_before) / dt;

        // Get new observation
        let next_state = self._get_obs()?;

        // Calculate reward
        let reward = self._calculate_reward(x_velocity, &action)?;

        // Swimmer doesn't terminate based on health
        let terminated = false;
        let truncated = self.steps >= self.config.max_episode_steps;

        // Create info dict
        let mut info = HashMap::new();
        info.insert("x_position".to_string(), x_position_after);

        let ctrl_cost = self._control_cost(&action)?;
        info.insert(
            "reward_forward".to_string(),
            x_velocity * self.config.forward_reward_weight,
        );
        info.insert("reward_ctrl".to_string(), -ctrl_cost);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            info,
            terminal,
            self.steps as u32,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        // Swimmer doesn't terminate based on health
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_with_zero_noise_scale_does_not_panic() {
        let config = SwimmerConfig {
            reset_noise_scale: 0.0,
            ..SwimmerConfig::default()
        };
        let mut env = MujocoSwimmerEnv::new(Some(config)).unwrap();
        assert!(env.reset(None).is_ok());
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = SwimmerConfig {
            max_episode_steps: 5,
            ..SwimmerConfig::default()
        };
        let mut env = MujocoSwimmerEnv::new(Some(config)).unwrap();
        env.reset(None).unwrap();

        let action = DVector::zeros(env.env.nu());
        for _ in 0..4 {
            let exp = env.step(action.clone()).unwrap();
            assert!(!exp.terminal.is_truncated());
        }
        let exp = env.step(action).unwrap();
        assert!(exp.terminal.is_truncated());
        assert!(env.is_truncated());
    }
}
