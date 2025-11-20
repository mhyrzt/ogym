use super::{config::HalfCheetahConfig, env::MujocoHalfCheetahEnv};
use crate::env::{
    environment::{Environment, Error, Experience, Terminal},
    mujoco::mjenv::MjEnv,
};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoHalfCheetahEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
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
        for i in 0..qpos.len() {
            qpos[i] += rng.random_range(noise_low..noise_high);
        }

        // Add noise to velocities
        let mut qvel = self.init_qvel.clone();
        for i in 0..qvel.len() {
            qvel[i] += self.config.reset_noise_scale * rng.random::<f64>() * 2.0
                - self.config.reset_noise_scale; // Standard normal would be more complex
        }

        self.env.set_state(&qpos, &qvel)?;

        let observation = self._get_observation()?;
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
        let x_pos_before = self.env.qpos()[0];

        // Apply action and simulate
        self.env.do_simulation(action.as_slice())?;

        // Get position after simulation
        let x_pos_after = self.env.qpos()[0];

        // Calculate velocity
        let dt = self.env.dt();
        let x_vel = (x_pos_after - x_pos_before) / dt;

        // Get new observation
        let next_state = self._get_observation()?;

        // Calculate reward
        let (reward, reward_info) = self._compute_reward(x_vel, &action)?;

        // For half-cheetah, there's no termination based on health, so always false
        let terminated = false;
        let truncated = self.env.time() > 1000.0; // Common truncation condition

        // Create info dict
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), x_pos_after);
        info.insert("x_velocity".to_string(), x_vel);
        info.extend(reward_info);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        // HalfCheetah doesn't terminate based on health
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        // For now, using a simple time-based truncation
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_observation()
    }
}

impl MujocoHalfCheetahEnv {
    // Helper methods
    fn _get_observation(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        let mut position = self.env.qpos().to_vec();
        let velocity = self.env.qvel().to_vec();

        if self.config.exclude_current_positions_from_observation {
            // Skip first element (x position)
            position = position[1..].to_vec();
        }

        observation.extend_from_slice(&position);
        observation.extend_from_slice(&velocity);

        Ok(DVector::from_vec(observation))
    }

    fn _compute_reward(
        &self,
        x_velocity: f64,
        action: &DVector<f64>,
    ) -> Result<(f64, std::collections::HashMap<String, f64>), Error> {
        let forward_reward = self.config.forward_reward_weight * x_velocity;
        let ctrl_cost = self._control_cost(action)?;
        let reward = forward_reward - ctrl_cost;

        let mut reward_info = std::collections::HashMap::new();
        reward_info.insert("reward_forward".to_string(), forward_reward);
        reward_info.insert("reward_ctrl".to_string(), -ctrl_cost);

        Ok((reward, reward_info))
    }

    fn _control_cost(&self, action: &DVector<f64>) -> Result<f64, Error> {
        let squared_actions: f64 = action.iter().map(|x| x * x).sum();
        Ok(self.config.ctrl_cost_weight * squared_actions)
    }

    fn _get_reset_info(&self) -> Result<Info, Error> {
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        Ok(info)
    }
}
