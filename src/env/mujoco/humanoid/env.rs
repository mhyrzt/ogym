use super::config::HumanoidConfig;
use crate::env::environment::{Environment, Experience, Terminal};
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use nalgebra::{DVector, Vector2};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoHumanoidEnv {
    pub(crate) env: MjEnv,
    pub(crate) config: HumanoidConfig,
    pub(crate) init_qpos: Vec<f64>,
    pub(crate) init_qvel: Vec<f64>,
    pub(crate) rng: StdRng,
    pub(crate) step_count: usize,
}

impl MujocoHumanoidEnv {
    pub fn new(config: Option<HumanoidConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(config.xml(), config.frame_skip())?;

        let init_qpos = env.init_qpos().to_vec();
        let init_qvel = env.init_qvel().to_vec();

        Ok(Self {
            init_qpos,
            init_qvel,
            config,
            env,
            rng: StdRng::from_os_rng(),
            step_count: 0,
        })
    }

    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let qpos = self.env.qpos();
        let qvel = self.env.qvel();

        let skip_pos = if self.config.exclude_current_positions_from_observation {
            2
        } else {
            0
        };
        let position_iter = qpos.iter().skip(skip_pos);

        let velocity_iter = qvel.iter();

        let cinert_iter = self
            .config
            .include_cinert_in_observation
            .then(|| self.env.cinert().iter().skip(1).flatten())
            .into_iter()
            .flatten();

        let cvel_iter = self
            .config
            .include_cvel_in_observation
            .then(|| self.env.cvel().iter().skip(1).flatten())
            .into_iter()
            .flatten();

        let qfrc_iter = self
            .config
            .include_qfrc_actuator_in_observation
            .then(|| self.env.qfrc_actuator().iter().skip(6))
            .into_iter()
            .flatten();

        let cfrc_ext_iter = self
            .config
            .include_cfrc_ext_in_observation
            .then(|| self.env.cfrc_ext().iter().skip(1).flatten())
            .into_iter()
            .flatten();

        let observation: Vec<f64> = position_iter
            .chain(velocity_iter)
            .chain(cinert_iter)
            .chain(cvel_iter)
            .chain(qfrc_iter)
            .chain(cfrc_ext_iter)
            .cloned()
            .collect();

        Ok(DVector::from_vec(observation))
    }

    fn _calculate_reward(
        &self,
        x_velocity: f64,
        action: &DVector<f64>,
    ) -> Result<(f64, HashMap<String, f64>), Error> {
        let forward_reward = self.config.forward_reward_weight * x_velocity;

        let healthy_reward = if self.is_healthy()? {
            self.config.healthy_reward
        } else {
            0.0
        };

        let ctrl_cost = self._control_cost(action)?;
        let contact_cost = self._contact_cost()?;

        let reward = forward_reward + healthy_reward - ctrl_cost - contact_cost;

        let mut info = HashMap::new();
        info.insert("reward_survive".to_string(), healthy_reward);
        info.insert("reward_forward".to_string(), forward_reward);
        info.insert("reward_ctrl".to_string(), -ctrl_cost);
        info.insert("reward_contact".to_string(), -contact_cost);

        Ok((reward, info))
    }

    fn _control_cost(&self, _action: &DVector<f64>) -> Result<f64, Error> {
        let sum_sq_ctrl: f64 = self.env.ctrl().iter().map(|&x| x.powi(2)).sum();

        Ok(self.config.ctrl_cost_weight * sum_sq_ctrl)
    }

    fn _contact_cost(&self) -> Result<f64, Error> {
        let sum_sq_force: f64 = self
            .env
            .cfrc_ext()
            .iter()
            .flatten()
            .map(|&f| f.powi(2))
            .sum();

        let cost = self.config.contact_cost_weight * sum_sq_force;
        Ok(cost.clamp(
            self.config.contact_cost_range.0,
            self.config.contact_cost_range.1,
        ))
    }

    pub fn is_healthy(&self) -> Result<bool, Error> {
        let z_pos = self.env.qpos()[2];
        let (min_z, max_z) = self.config.healthy_z_range;
        Ok(z_pos > min_z && z_pos < max_z)
    }

    fn _mass_center(&self) -> Result<Vector2<f64>, Error> {
        let body_mass = self.env.body_mass();
        let xipos = self.env.xipos();

        let (mass_x, mass_y, total_mass) = body_mass
            .iter()
            .zip(xipos.iter())
            .fold((0.0, 0.0, 0.0), |acc, (&mass, &pos)| {
                (acc.0 + mass * pos[0], acc.1 + mass * pos[1], acc.2 + mass)
            });

        if total_mass > 0.0 {
            Ok(Vector2::new(mass_x / total_mass, mass_y / total_mass))
        } else {
            Ok(Vector2::new(0.0, 0.0))
        }
    }
}

type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoHumanoidEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        if let Some(s) = seed {
            self.rng = StdRng::seed_from_u64(s);
        }

        self.env.reset_to_initial()?;
        self.step_count = 0;

        let noise_low = -self.config.reset_noise_scale;
        let noise_high = self.config.reset_noise_scale;

        let qpos: Vec<f64> = self
            .init_qpos
            .iter()
            .map(|&x| x + self.rng.random_range(noise_low..noise_high))
            .collect();

        let qvel: Vec<f64> = self
            .init_qvel
            .iter()
            .map(|&x| x + self.rng.random_range(noise_low..noise_high))
            .collect();

        self.env.set_state(&qpos, &qvel)?;

        let obs = self._get_obs()?;

        let mut info = HashMap::new();
        info.insert("z_position".to_string(), self.env.qpos()[2]);

        Ok((obs, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let curr_state = self.state()?;
        let xy_pos_before = self._mass_center()?;

        self.env.do_simulation(action.as_slice())?;
        self.step_count += 1;

        let xy_pos_after = self._mass_center()?;
        let dt = self.env.dt();
        let xy_velocity = (xy_pos_after - xy_pos_before) / dt;
        let x_velocity = xy_velocity.x;

        let next_state = self._get_obs()?;
        let (reward, reward_info) = self._calculate_reward(x_velocity, &action)?;

        let is_healthy = self.is_healthy()?;
        let terminated = self.config.terminate_when_unhealthy && !is_healthy;

        let truncated = self.step_count >= self.config.max_episode_steps;

        let mut info = reward_info;
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert("z_position".to_string(), self.env.qpos()[2]);
        info.insert("x_velocity".to_string(), x_velocity);
        info.insert("y_velocity".to_string(), xy_velocity.y);

        let ten_len_sum: f64 = self.env.ten_length().iter().sum();
        let ten_vel_sum: f64 = self.env.ten_velocity().iter().sum();
        info.insert("tendon_length_sum".to_string(), ten_len_sum);
        info.insert("tendon_velocity_sum".to_string(), ten_vel_sum);

        let terminal_flags = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            info,
            terminal_flags,
            self.step_count as u32,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        let unhealthy = !self.is_healthy()?;
        Ok(self.config.terminate_when_unhealthy && unhealthy)
    }

    fn is_truncated(&self) -> bool {
        self.step_count >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity_initialization() {
        let config = HumanoidConfig::default().with_frame_skip(2);
        let env_res = MujocoHumanoidEnv::new(Some(config));
        assert!(
            env_res.is_ok(),
            "Environment should initialize with valid config"
        );

        let env = env_res.unwrap();
        assert_eq!(env.config.frame_skip, 2, "Config settings should persist");
        assert!(!env.init_qpos.is_empty(), "Initial QPOS should be loaded");
    }

    #[test]
    fn test_reset_noise_functional() {
        let config = HumanoidConfig::default().with_reset_noise_scale(1.0);
        let mut env = MujocoHumanoidEnv::new(Some(config)).unwrap();

        let (_, _) = env.reset(Some(1)).unwrap();
        let start_pos_1 = env.env.qpos().to_vec();

        let (_, _) = env.reset(Some(2)).unwrap();
        let start_pos_2 = env.env.qpos().to_vec();

        assert_ne!(
            start_pos_1, start_pos_2,
            "Different seeds should produce different noise"
        );

        let (_, _) = env.reset(Some(1)).unwrap();
        let start_pos_3 = env.env.qpos().to_vec();

        let diff: f64 = start_pos_1
            .iter()
            .zip(start_pos_3.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        assert!(
            diff < 1e-9,
            "Same seed should produce identical starting state"
        );
    }

    #[test]
    fn test_observation_dimensions() {
        let config =
            HumanoidConfig::default().with_observation_settings(true, true, true, true, true);

        let mut env = MujocoHumanoidEnv::new(Some(config)).unwrap();
        let (obs, _) = env.reset(None).unwrap();

        assert!(!obs.is_empty(), "Observation vector should not be empty");
    }

    #[test]
    fn test_step_cycle_and_physics() {
        let mut env = MujocoHumanoidEnv::new(None).unwrap();
        env.reset(None).unwrap();

        let action_dim = env.env.nu();
        let action = DVector::from_element(action_dim, 0.5);

        let step_res = env.step(action);
        assert!(step_res.is_ok());

        let exp = step_res.unwrap();

        assert!(env.env.time() > 0.0);
        assert_eq!(env.step_count, 1);

        assert!(exp.reward.is_finite());

        assert!(!exp.terminal.is_terminated());
    }

    #[test]
    fn test_truncation() {
        let config = HumanoidConfig::default().with_max_episode_steps(2);
        let mut env = MujocoHumanoidEnv::new(Some(config)).unwrap();
        env.reset(None).unwrap();

        let action = DVector::zeros(env.env.nu());

        let exp1 = env.step(action.clone()).unwrap();
        assert!(!exp1.terminal.is_truncated());

        let exp2 = env.step(action).unwrap();
        assert!(
            exp2.terminal.is_truncated(),
            "Environment should truncate at max steps"
        );
    }

    #[test]
    fn test_functional_center_of_mass() {
        let env = MujocoHumanoidEnv::new(None).unwrap();
        let com_res = env._mass_center();
        assert!(com_res.is_ok());
        let com = com_res.unwrap();

        assert!(com.x.abs() < 1.0);
        assert!(com.y.abs() < 1.0);
    }
}
