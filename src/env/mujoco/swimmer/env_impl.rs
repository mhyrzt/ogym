use super::{config::SwimmerConfig, env::MujocoSwimmerEnv};
use crate::env::{
    environment::{Environment, Error, Experience, Terminal},
    mujoco::mjenv::MjEnv,
};
use nalgebra::DVector;
use rand::{Rng, SeedableRng, rngs::StdRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoSwimmerEnv {
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
            qpos[i] += rng.gen_range(noise_low..noise_high);
        }

        // Add noise to velocities
        let mut qvel = self.init_qvel.clone();
        for i in 0..qvel.len() {
            qvel[i] += rng.gen_range(noise_low..noise_high);
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
        let truncated = self.env.time() > 1000.0; // Time-based truncation

        // Create info dict
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), x_position_after);

        let ctrl_cost = self._control_cost(&action)?;
        info.insert(
            "reward_forward".to_string(),
            x_velocity * self.config.forward_reward_weight,
        );
        info.insert("reward_ctrl".to_string(), -ctrl_cost);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        // Swimmer doesn't terminate based on health
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        // Time-based truncation
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

impl MujocoSwimmerEnv {
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

    fn _get_reset_info(&self) -> Result<Info, Error> {
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        Ok(info)
    }
}
