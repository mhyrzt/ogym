use super::env::MujocoPusherEnv;
use crate::env::environment::{Environment, Error, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoPusherEnv {
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

        let noise_low = -0.01;
        let noise_high = 0.01;

        let mut qpos = self.init_qpos.clone();
        for i in 0..qpos.len() {
            qpos[i] += rng.random_range(noise_low..noise_high);
        }

        let mut qvel = self.init_qvel.clone();
        for i in 0..qvel.len() {
            qvel[i] += rng.random_range(noise_low..noise_high);
        }

        self.env.set_state(&qpos, &qvel)?;

        let observation = self._get_obs()?;

        let info = std::collections::HashMap::new();

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let curr_state = self.state()?;

        self.env.do_simulation(action.as_slice())?;

        let next_state = self._get_obs()?;

        let (reward, reward_info) = self._get_rew(&action)?;

        let terminated = false;
        let truncated = self.env.time() > 1000.0; // Time-based truncation

        let mut info = std::collections::HashMap::new();
        info.extend(reward_info);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal, 0,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

impl MujocoPusherEnv {
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        observation.extend_from_slice(self.env.qpos());
        observation.extend_from_slice(self.env.qvel());

        let object_pos = self._get_object_pos()?;
        observation.extend_from_slice(&[object_pos[0], object_pos[1], object_pos[2]]);

        let target_pos = self._get_target_pos()?;
        observation.extend_from_slice(&[target_pos[0], target_pos[1], target_pos[2]]);

        let fingertip_pos = self._get_fingertip_pos()?;
        observation.extend_from_slice(&[fingertip_pos[0], fingertip_pos[1], fingertip_pos[2]]);

        Ok(DVector::from_vec(observation))
    }

    fn _get_rew(
        &self,
        action: &DVector<f64>,
    ) -> Result<(f64, std::collections::HashMap<String, f64>), Error> {
        let object_pos = self._get_object_pos()?;
        let target_pos = self._get_target_pos()?;
        let fingertip_pos = self._get_fingertip_pos()?;

        let dist_fingertip_object = nalgebra::Vector3::new(
            object_pos[0] - fingertip_pos[0],
            object_pos[1] - fingertip_pos[1],
            object_pos[2] - fingertip_pos[2],
        )
        .norm();

        let dist_object_target = nalgebra::Vector3::new(
            target_pos[0] - object_pos[0],
            target_pos[1] - object_pos[1],
            target_pos[2] - object_pos[2],
        )
        .norm();

        let reward_near = -self.config.reward_near_weight * dist_fingertip_object;
        let reward_dist = -self.config.reward_dist_weight * dist_object_target;
        let reward_ctrl = -self.config.reward_control_weight * action.norm_squared();

        let total_reward = reward_near + reward_dist + reward_ctrl;

        let mut reward_info = std::collections::HashMap::new();
        reward_info.insert("reward_near".to_string(), reward_near);
        reward_info.insert("reward_dist".to_string(), reward_dist);
        reward_info.insert("reward_ctrl".to_string(), reward_ctrl);

        Ok((total_reward, reward_info))
    }

    fn _get_object_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        if self.env.nbody() > 3 && self.env.xipos().len() > 3 {
            let object_data = self.env.xipos()[3]; // Example: assuming object is at index 3
            Ok(nalgebra::Vector3::new(
                object_data[0],
                object_data[1],
                object_data[2],
            ))
        } else {
            Ok(nalgebra::Vector3::new(0.0, 0.0, 0.0))
        }
    }

    fn _get_target_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        // In a real implementation, we'd find the target body by name and get its position
        // For now, returning a default value
        if self.env.nbody() > 4 && self.env.xipos().len() > 4 {
            let target_data = self.env.xipos()[4]; // Example: assuming target is at index 4
            Ok(nalgebra::Vector3::new(
                target_data[0],
                target_data[1],
                target_data[2],
            ))
        } else {
            Ok(nalgebra::Vector3::new(0.0, 0.0, 0.0))
        }
    }

    fn _get_fingertip_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        if self.env.nbody() > 2 && self.env.xipos().len() > 2 {
            let fingertip_data = self.env.xipos()[2];
            Ok(nalgebra::Vector3::new(
                fingertip_data[0],
                fingertip_data[1],
                fingertip_data[2],
            ))
        } else {
            Ok(nalgebra::Vector3::new(0.0, 0.0, 0.0))
        }
    }

    fn _get_reset_info(&self) -> Result<Info, Error> {
        Ok(std::collections::HashMap::new())
    }
}
