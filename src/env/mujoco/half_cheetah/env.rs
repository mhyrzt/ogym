use super::config::HalfCheetahConfig;
use crate::env::environment::{Environment, Experience, Terminal};
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub type Action = DVector<f64>;
pub type State = DVector<f64>;
pub type Info = HashMap<String, f64>;

pub struct MujocoHalfCheetahEnv {
    pub env: MjEnv,
    config: HalfCheetahConfig,
    init_qpos: Vec<f64>,
    init_qvel: Vec<f64>,
    step_count: usize,
}

impl MujocoHalfCheetahEnv {
    pub fn new(config: Option<HalfCheetahConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(config.xml(), config.frame_skip())?;
        let init_qpos = env.init_qpos().to_vec();
        let init_qvel = env.init_qvel().to_vec();

        Ok(Self {
            env,
            config,
            init_qpos,
            init_qvel,
            step_count: 0,
        })
    }

    fn _get_observation(&self) -> Result<State, Error> {
        let qpos = self.env.qpos();
        let qvel = self.env.qvel();

        let pos_start_idx = if self.config.exclude_current_positions_from_observation() {
            1
        } else {
            0
        };

        let obs_dim = (qpos.len() - pos_start_idx) + qvel.len();
        let mut observation = Vec::with_capacity(obs_dim);

        observation.extend_from_slice(&qpos[pos_start_idx..]);
        observation.extend_from_slice(qvel);

        Ok(DVector::from_vec(observation))
    }

    fn _compute_reward(&self, x_velocity: f64, action: &Action) -> (f64, Info) {
        let forward_reward = self.config.forward_reward_weight() * x_velocity;

        let squared_action_sum: f64 = action.iter().map(|a| a.powi(2)).sum();
        let ctrl_cost = self.config.ctrl_cost_weight() * squared_action_sum;

        let total_reward = forward_reward - ctrl_cost;

        let mut info = HashMap::new();
        info.insert("reward_forward".to_string(), forward_reward);
        info.insert("reward_ctrl".to_string(), -ctrl_cost);

        (total_reward, info)
    }

    fn _get_reset_info(&self) -> Info {
        let mut info = HashMap::new();
        if let Some(x_pos) = self.env.qpos().first() {
            info.insert("x_position".to_string(), *x_pos);
        }
        info
    }
}

impl Environment for MujocoHalfCheetahEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.step_count = 0;
        self.env.reset_to_initial()?;

        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };

        let noise_scale = self.config.reset_noise_scale();

        let qpos_noisy: Vec<f64> = if noise_scale > 0.0 {
            self.init_qpos
                .iter()
                .map(|&val| {
                    let noise = rng.random_range(-noise_scale..noise_scale);
                    val + noise
                })
                .collect()
        } else {
            self.init_qpos.clone()
        };

        let qvel_noisy: Vec<f64> = if noise_scale > 0.0 {
            self.init_qvel
                .iter()
                .map(|&val| {
                    let noise = rng.random_range(-noise_scale..noise_scale);
                    val + noise
                })
                .collect()
        } else {
            self.init_qvel.clone()
        };

        self.env.set_state(&qpos_noisy, &qvel_noisy)?;

        Ok((self._get_observation()?, self._get_reset_info()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let current_state = self._get_observation()?;
        let x_pos_before = self.env.qpos()[0];

        self.env.do_simulation(action.as_slice())?;
        self.step_count += 1;

        let x_pos_after = self.env.qpos()[0];

        let dt = self.env.dt();
        let x_velocity = (x_pos_after - x_pos_before) / dt;
        let (reward, mut info) = self._compute_reward(x_velocity, &action);

        let next_state = self._get_observation()?;

        let terminated = false;
        let truncated = self.env.time() > 1000.0;

        info.insert("x_position".to_string(), x_pos_after);
        info.insert("x_velocity".to_string(), x_velocity);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            current_state,
            reward,
            action,
            next_state,
            info,
            terminal,
            self.step_count as u32,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_observation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const TEST_XML: &str = r#"
    <mujoco model="cheetah_test">
        <compiler angle="radian"/>
        <option timestep="0.01" gravity="0 0 -9.81"/>
        <worldbody>
            <body name="torso" pos="0 0 0">
                <joint name="rootx" type="slide" axis="1 0 0" />
                <joint name="rootz" type="slide" axis="0 0 1" />
                <joint name="rooty" type="hinge" axis="0 1 0" />
                <geom type="capsule" size="0.046" fromto="-.5 0 0 .5 0 0" />
            </body>
        </worldbody>
        <actuator>
             <motor name="m1" joint="rootx" gear="1" />
        </actuator>
    </mujoco>
    "#;

    fn get_test_config() -> HalfCheetahConfig {
        HalfCheetahConfig::new()
            .with_xml(TEST_XML)
            .with_frame_skip(1)
            .with_reset_noise_scale(0.0)
    }

    #[test]
    fn test_env_initialization() {
        let config = get_test_config();
        let env = MujocoHalfCheetahEnv::new(Some(config));
        assert!(env.is_ok(), "Environment should initialize with valid XML");

        let env = env.unwrap();
        assert_eq!(env.env.nq(), 3);
        assert_eq!(env.env.nu(), 1);
    }

    #[test]
    fn test_reset_determinism_and_noise() {
        let mut config = get_test_config();
        config = config.with_reset_noise_scale(0.0);
        let mut env = MujocoHalfCheetahEnv::new(Some(config.clone())).unwrap();

        let (obs1, _) = env.reset(Some(42)).unwrap();
        let (obs2, _) = env.reset(Some(99)).unwrap();

        assert_relative_eq!(obs1[0], env.init_qpos[1]);
        assert_eq!(obs1, obs2);

        config = config.with_reset_noise_scale(0.5);
        let mut env_noisy = MujocoHalfCheetahEnv::new(Some(config)).unwrap();

        let (obs_seed_a, _) = env_noisy.reset(Some(123)).unwrap();
        let (obs_seed_a_again, _) = env_noisy.reset(Some(123)).unwrap();
        let (obs_seed_b, _) = env_noisy.reset(Some(456)).unwrap();
        assert_relative_eq!(obs_seed_a, obs_seed_a_again);
        assert_ne!(obs_seed_a, obs_seed_b);
    }

    #[test]
    fn test_observation_dimensions() {
        let config = get_test_config().with_position_exclusion(true);
        let mut env = MujocoHalfCheetahEnv::new(Some(config)).unwrap();
        let (obs, _) = env.reset(None).unwrap();

        assert_eq!(obs.len(), 5, "Observation vector dimension mismatch");
    }

    #[test]
    fn test_step_logic_and_reward() {
        let config = get_test_config()
            .with_forward_reward_weight(1.0)
            .with_ctrl_cost_weight(0.1);

        let mut env = MujocoHalfCheetahEnv::new(Some(config)).unwrap();
        env.reset(Some(1)).unwrap();

        let action = DVector::from_vec(vec![1.0]);
        let experience = env.step(action.clone()).unwrap();

        let next_x = experience.info.get("x_position").unwrap();
        assert!(
            *next_x > 0.0,
            "Agent should move forward with positive action"
        );

        let x_vel = experience.info.get("x_velocity").unwrap();
        let fwd_reward = 1.0 * x_vel;
        let ctrl_cost = 0.1 * (1.0f64.powi(2));
        let expected_reward = fwd_reward - ctrl_cost;

        assert_relative_eq!(experience.reward, expected_reward, epsilon = 1e-6);
        assert!(!experience.terminal.is_terminated());
    }

    #[test]
    fn test_action_bounds_check() {
        let mut env = MujocoHalfCheetahEnv::new(Some(get_test_config())).unwrap();
        env.reset(None).unwrap();

        let bad_action = DVector::from_vec(vec![1.0, 2.0, 3.0]);
        let result = env.step(bad_action);

        assert!(
            result.is_err(),
            "Should return error for wrong action dimension"
        );
    }
}
