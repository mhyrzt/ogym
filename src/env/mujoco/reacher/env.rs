use super::config::ReacherConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use crate::env::environment::{Environment, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoReacherEnv {
    pub env: MjEnv,
    pub config: ReacherConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
    steps: usize,
}

impl MujocoReacherEnv {
    pub fn new(config: Option<ReacherConfig>) -> Result<Self, Error> {
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
        // Matches Gymnasium's Reacher-v4 exactly: cos/sin of the two arm
        // joint angles (not the raw angles), the target's own joint
        // position, the arm's joint velocities only (not the target's), and
        // the 3D fingertip-to-target vector (not raw absolute positions).
        let theta = &self.env.qpos()[0..2];

        let mut observation = Vec::new();
        observation.push(theta[0].cos());
        observation.push(theta[1].cos());
        observation.push(theta[0].sin());
        observation.push(theta[1].sin());
        observation.extend_from_slice(&self.env.qpos()[2..4]);
        observation.extend_from_slice(&self.env.qvel()[0..2]);

        let fingertip_pos = self._get_fingertip_pos()?;
        let target_pos = self._get_target_pos()?;
        observation.push(fingertip_pos[0] - target_pos[0]);
        observation.push(fingertip_pos[1] - target_pos[1]);
        observation.push(fingertip_pos[2] - target_pos[2]);

        Ok(DVector::from_vec(observation))
    }

    fn _get_rew(
        &self,
        action: &DVector<f64>,
    ) -> Result<(f64, HashMap<String, f64>), Error> {
        let target_pos = self._get_target_pos()?;
        let fingertip_pos = self._get_fingertip_pos()?;
        let dist = (fingertip_pos - target_pos).norm();

        let reward_dist = -self.config.reward_dist_weight * dist;
        let reward_ctrl = -self.config.reward_control_weight * action.norm_squared();

        let total_reward = reward_dist + reward_ctrl;

        let mut reward_info = HashMap::new();
        reward_info.insert("reward_dist".to_string(), reward_dist);
        reward_info.insert("reward_ctrl".to_string(), reward_ctrl);

        Ok((total_reward, reward_info))
    }

    fn _get_target_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        self.env
            .body_com_vector("target")
            .ok_or_else(|| Error::MjInitError("body 'target' not found in model".to_string()))
    }

    fn _get_fingertip_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        self.env
            .body_com_vector("fingertip")
            .ok_or_else(|| Error::MjInitError("body 'fingertip' not found in model".to_string()))
    }

    fn _get_reset_info(&self) -> Result<HashMap<String, f64>, Error> {
        Ok(HashMap::new())
    }
}

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoReacherEnv {
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
        let info = HashMap::new();

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
        self.steps += 1;

        // Get new observation
        let next_state = self._get_obs()?;

        // Calculate reward
        let (reward, reward_info) = self._get_rew(&action)?;

        // For reacher, no termination based on health
        let terminated = false;
        let truncated = self.steps >= self.config.max_episode_steps;

        // Create info dict
        let mut info = HashMap::new();
        info.extend(reward_info);

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
        // Reacher doesn't terminate based on health
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
    fn test_target_body_resolves_to_target_joints_not_arm_links() {
        // qpos order (from model.xml): [joint0, joint1, target_x, target_y].
        // Move only the target's slide joints and confirm the resolved
        // "target" body moves by exactly that delta, while "fingertip"
        // (which only depends on the arm joints) doesn't move at all. This
        // directly catches the original bug, where both were read from
        // hard-coded indices landing on the arm-link bodies instead: under
        // that bug, target's position was insensitive to target_x/target_y.
        let mut env = MujocoReacherEnv::new(None).unwrap();
        let qvel = vec![0.0, 0.0, 0.0, 0.0];

        env.env.set_state(&vec![0.0, 0.0, 0.0, 0.0], &qvel).unwrap();
        let target_at_zero = env._get_target_pos().unwrap();
        let fingertip_at_zero = env._get_fingertip_pos().unwrap();

        env.env.set_state(&vec![0.0, 0.0, 0.15, -0.15], &qvel).unwrap();
        let target_after_move = env._get_target_pos().unwrap();
        let fingertip_after_move = env._get_fingertip_pos().unwrap();

        assert!((target_after_move.x - target_at_zero.x - 0.15).abs() < 1e-6);
        assert!((target_after_move.y - target_at_zero.y - (-0.15)).abs() < 1e-6);
        assert!((fingertip_after_move - fingertip_at_zero).norm() < 1e-9);
    }

    #[test]
    fn test_observation_dimension_matches_config() {
        let env = MujocoReacherEnv::new(None).unwrap();
        let obs = env.state().unwrap();
        assert_eq!(obs.len(), env.config.observation_shape.0);
        assert_eq!(obs.len(), 11);
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = ReacherConfig {
            max_episode_steps: 5,
            ..ReacherConfig::default()
        };
        let mut env = MujocoReacherEnv::new(Some(config)).unwrap();
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

    #[test]
    fn test_reward_uses_correct_body_positions() {
        let env = MujocoReacherEnv::new(None).unwrap();
        let action = DVector::from_element(2, 0.0);

        let (reward, info) = env._get_rew(&action).unwrap();

        let target_pos = env._get_target_pos().unwrap();
        let fingertip_pos = env._get_fingertip_pos().unwrap();
        let expected_dist = -env.config.reward_dist_weight * (fingertip_pos - target_pos).norm();

        assert!((info["reward_dist"] - expected_dist).abs() < 1e-9);
        assert!((reward - expected_dist).abs() < 1e-9);
    }
}
