use super::env::MujocoReacherEnv;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoReacherEnv {
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

        let noise_low = -0.1; // Default noise for reacher
        let noise_high = 0.1;

        // Add noise to positions
        let mut qpos = self.init_qpos.clone();
        for i in 0..qpos.len() {
            qpos[i] += rng.random_range(noise_low..noise_high);
        }

        // Add noise to velocities
        let mut qvel = self.init_qvel.clone();
        for i in 0..qvel.len() {
            qvel[i] += rng.random_range(noise_low..noise_high);
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

        // Calculate reward
        let (reward, reward_info) = self._get_rew(&action)?;

        // For reacher, no termination based on health
        let terminated = false;
        let truncated = self.env.time() > 1000.0; // Time-based truncation

        // Create info dict
        let mut info = std::collections::HashMap::new();
        info.extend(reward_info);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        // Reacher doesn't terminate based on health
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        // For now, using a simple time-based truncation
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

impl MujocoReacherEnv {
    // Helper methods
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        // Position (joint angles)
        observation.extend_from_slice(self.env.qpos());

        // Velocity (joint velocities)
        observation.extend_from_slice(self.env.qvel());

        // Get target position (assuming it's stored somehow in the model)
        // This is simplified - in real implementation, we'd look up target position
        let target_pos = self._get_target_pos()?;
        observation.extend_from_slice(&[target_pos[0], target_pos[1]]);

        // Get fingertip position
        let fingertip_pos = self._get_fingertip_pos()?;
        observation.extend_from_slice(&[fingertip_pos[0], fingertip_pos[1]]);

        Ok(DVector::from_vec(observation))
    }

    fn _get_rew(
        &self,
        action: &DVector<f64>,
    ) -> Result<(f64, std::collections::HashMap<String, f64>), Error> {
        // Calculate distance to target
        let target_pos = self._get_target_pos()?;
        let fingertip_pos = self._get_fingertip_pos()?;

        let dist_vec = nalgebra::Vector2::new(
            target_pos[0] - fingertip_pos[0],
            target_pos[1] - fingertip_pos[1],
        );
        let dist = dist_vec.norm();

        let reward_dist = -self.config.reward_dist_weight * dist;
        let reward_ctrl = -self.config.reward_control_weight * action.norm_squared();

        let total_reward = reward_dist + reward_ctrl;

        let mut reward_info = std::collections::HashMap::new();
        reward_info.insert("reward_dist".to_string(), reward_dist);
        reward_info.insert("reward_ctrl".to_string(), reward_ctrl);

        Ok((total_reward, reward_info))
    }

    fn _get_target_pos(&self) -> Result<nalgebra::Vector2<f64>, Error> {
        // Get target body position
        // In a real implementation, we'd find the target body by name and get its position
        // For now, returning a default value
        if self.env.nbody() > 1 && self.env.xipos().len() > 1 {
            // Assuming target body is the second one (index 1)
            let target_data = self.env.xipos()[1];
            Ok(nalgebra::Vector2::new(target_data[0], target_data[1]))
        } else {
            Ok(nalgebra::Vector2::new(0.0, 0.0))
        }
    }

    fn _get_fingertip_pos(&self) -> Result<nalgebra::Vector2<f64>, Error> {
        // Get fingertip body position
        // In a real implementation, we'd find the fingertip body by name and get its position
        // For now, returning a default value - in the actual model, fingertip would be a specific body
        if self.env.nbody() > 2 && self.env.xipos().len() > 2 {
            // Assuming fingertip body is the third one (index 2) for example
            let fingertip_data = self.env.xipos()[2];
            Ok(nalgebra::Vector2::new(fingertip_data[0], fingertip_data[1]))
        } else {
            Ok(nalgebra::Vector2::new(0.0, 0.0))
        }
    }

    fn _get_reset_info(&self) -> Result<Info, Error> {
        Ok(std::collections::HashMap::new())
    }
}
