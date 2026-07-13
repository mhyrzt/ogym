use super::config::PusherConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use crate::env::environment::{Environment, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoPusherEnv {
    pub env: MjEnv,
    pub config: PusherConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
    steps: usize,
}

impl MujocoPusherEnv {
    pub fn new(config: Option<PusherConfig>) -> Result<Self, Error> {
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

    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        // Only the 7 arm joints, matching Gymnasium's Pusher-v4: the object
        // and goal slide-joint positions are redundant with their body COMs
        // below, so they're excluded (unlike Ant/HalfCheetah/etc., this env
        // never includes them at all, not even optionally).
        observation.extend_from_slice(&self.env.qpos()[..7]);
        observation.extend_from_slice(&self.env.qvel()[..7]);

        let fingertip_pos = self._get_fingertip_pos()?;
        observation.extend_from_slice(&[fingertip_pos[0], fingertip_pos[1], fingertip_pos[2]]);

        let object_pos = self._get_object_pos()?;
        observation.extend_from_slice(&[object_pos[0], object_pos[1], object_pos[2]]);

        let target_pos = self._get_target_pos()?;
        observation.extend_from_slice(&[target_pos[0], target_pos[1], target_pos[2]]);

        Ok(DVector::from_vec(observation))
    }

    fn _get_rew(
        &self,
        action: &DVector<f64>,
    ) -> Result<(f64, HashMap<String, f64>), Error> {
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

        let mut reward_info = HashMap::new();
        reward_info.insert("reward_near".to_string(), reward_near);
        reward_info.insert("reward_dist".to_string(), reward_dist);
        reward_info.insert("reward_ctrl".to_string(), reward_ctrl);

        Ok((total_reward, reward_info))
    }

    fn _get_object_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        self.env
            .body_com_vector("object")
            .ok_or_else(|| Error::MjInitError("body 'object' not found in model".to_string()))
    }

    fn _get_target_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        self.env
            .body_com_vector("goal")
            .ok_or_else(|| Error::MjInitError("body 'goal' not found in model".to_string()))
    }

    fn _get_fingertip_pos(&self) -> Result<nalgebra::Vector3<f64>, Error> {
        self.env
            .body_com_vector("tips_arm")
            .ok_or_else(|| Error::MjInitError("body 'tips_arm' not found in model".to_string()))
    }

    fn _get_reset_info(&self) -> Result<HashMap<String, f64>, Error> {
        Ok(HashMap::new())
    }
}

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoPusherEnv {
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

        let info = HashMap::new();

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let curr_state = self.state()?;

        self.env.do_simulation(action.as_slice())?;
        self.steps += 1;

        let next_state = self._get_obs()?;

        let (reward, reward_info) = self._get_rew(&action)?;

        let terminated = false;
        let truncated = self.steps >= self.config.max_episode_steps;

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
    fn test_body_lookups_match_model_xml() {
        // model.xml declares: <body name="object" pos="0.45 -0.05 -0.275">
        // and <body name="goal" pos="0.45 -0.05 -0.3230">, both with only
        // slide joints starting at 0 displacement, so right after reset
        // their COM should sit at (approximately) their declared pos. This
        // guards against ever again reading these positions from the wrong
        // body index.
        let env = MujocoPusherEnv::new(None).unwrap();

        let object_pos = env._get_object_pos().unwrap();
        assert!((object_pos.x - 0.45).abs() < 1e-6);
        assert!((object_pos.y - (-0.05)).abs() < 1e-6);
        assert!((object_pos.z - (-0.275)).abs() < 1e-6);

        let target_pos = env._get_target_pos().unwrap();
        assert!((target_pos.x - 0.45).abs() < 1e-6);
        assert!((target_pos.y - (-0.05)).abs() < 1e-6);
        assert!((target_pos.z - (-0.3230)).abs() < 1e-6);

        // tips_arm depends on the 7-joint arm chain rather than a fixed pos
        // attribute, but it must resolve to a distinct body from object/goal.
        let fingertip_pos = env._get_fingertip_pos().unwrap();
        assert!((fingertip_pos - object_pos).norm() > 1e-3);
        assert!((fingertip_pos - target_pos).norm() > 1e-3);
    }

    #[test]
    fn test_observation_dimension_matches_config() {
        let env = MujocoPusherEnv::new(None).unwrap();
        let obs = env.state().unwrap();
        assert_eq!(obs.len(), env.config.observation_shape.0);
        assert_eq!(obs.len(), 23);
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = PusherConfig {
            max_episode_steps: 5,
            ..PusherConfig::default()
        };
        let mut env = MujocoPusherEnv::new(Some(config)).unwrap();
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
        let env = MujocoPusherEnv::new(None).unwrap();
        let action = DVector::from_element(7, 0.0);

        let (reward, info) = env._get_rew(&action).unwrap();

        let object_pos = env._get_object_pos().unwrap();
        let target_pos = env._get_target_pos().unwrap();
        let fingertip_pos = env._get_fingertip_pos().unwrap();

        let expected_near =
            -env.config.reward_near_weight * (object_pos - fingertip_pos).norm();
        let expected_dist =
            -env.config.reward_dist_weight * (target_pos - object_pos).norm();

        assert!((info["reward_near"] - expected_near).abs() < 1e-9);
        assert!((info["reward_dist"] - expected_dist).abs() < 1e-9);
        assert!((reward - (expected_near + expected_dist)).abs() < 1e-9);
    }
}
