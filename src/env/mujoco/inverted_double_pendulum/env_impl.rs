use super::env::MujocoInvertedDoublePendulumEnv;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoInvertedDoublePendulumEnv {
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

        // Create empty info dict as in the Python version
        let info = std::collections::HashMap::new();

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        // Store current state
        let curr_state = self.state()?;

        // Apply action and simulate
        self.env.do_simulation(action.as_slice())?;

        // Get new observation
        let next_state = self._get_obs()?;

        // Calculate reward (healthy reward only - no termination in inverted double pendulum)
        let reward = self._healthy_reward();

        // For inverted double pendulum, no termination based on health, but we check if it fell
        let terminated = self._has_fallen()?;
        let truncated = self.env.time() > self.config.max_steps as f64 * self.env.timestep(); // Time-based truncation

        // Create info dict
        let info = self._get_info()?;

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(self._has_fallen()?)
    }

    fn is_truncated(&self) -> bool {
        self.env.time() > (self.config.max_steps as f64) * self.env.timestep()
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

impl MujocoInvertedDoublePendulumEnv {
    // Helper methods
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        // Get cart position, velocity, and pole angles and velocities
        let mut observation = Vec::new();
        observation.extend_from_slice(self.env.qpos());
        observation.extend_from_slice(self.env.qvel());

        // Additional calculations for tip position if needed
        Ok(DVector::from_vec(observation))
    }

    fn _has_fallen(&self) -> Result<bool, Error> {
        // Inverted double pendulum is considered "fallen" when the tip drops below a certain height
        // The tip is typically the end of the second pole
        // For now, we'll check if the pendulum angle is too vertical
        let qpos = self.env.qpos();

        // The x, y of the tip can be computed from the joint angles
        // This is a simplified check - in actual implementation this would need more complex calculation
        let pole_angle1 = if qpos.len() > 1 { qpos[1] } else { 0.0 }; // First joint angle
        let pole_angle2 = if qpos.len() > 2 { qpos[2] } else { 0.0 }; // Second joint angle

        // Check if pole angles are within reasonable bounds
        // If angles are too wild, consider it fallen
        Ok(pole_angle1.abs() > 1.5 || pole_angle2.abs() > 1.5) // Adjust threshold as needed
    }

    fn _healthy_reward(&self) -> f64 {
        // Always return healthy reward unless fallen
        if !matches!(self._has_fallen(), Ok(true)) {
            self.config.healthy_reward
        } else {
            0.0
        }
    }

    fn _get_info(&self) -> Result<Info, Error> {
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("x_velocity".to_string(), self.env.qvel()[0]);
        Ok(info)
    }

    fn _get_reset_info(&self) -> Result<Info, Error> {
        Ok(std::collections::HashMap::new())
    }
}
