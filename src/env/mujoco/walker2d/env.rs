use super::config::Walker2dConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use crate::env::environment::{Environment, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoWalker2dEnv {
    pub env: MjEnv,
    pub config: Walker2dConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
}

impl MujocoWalker2dEnv {
    pub fn new(config: Option<Walker2dConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(&config.xml_file, config.frame_skip)?;
        Ok(Self {
            init_qpos: env.init_qpos().into(),
            init_qvel: env.init_qvel().into(),
            config,
            env,
        })
    }

    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        let mut position = self.env.qpos().to_vec();
        let velocity = self.env.qvel().to_vec();

        if self.config.exclude_current_positions_from_observation {
            position = position[1..].to_vec();
        }

        observation.extend_from_slice(&position);
        observation.extend_from_slice(&velocity);

        Ok(DVector::from_vec(observation))
    }

    fn _get_rew(&self, x_velocity: f64, action: &DVector<f64>) -> Result<f64, Error> {
        let forward_reward = self.config.forward_reward_weight * x_velocity;
        let healthy_reward = if self.is_healthy()? {
            self.config.healthy_reward
        } else {
            0.0
        };
        let ctrl_cost = self._control_cost(action)?;

        Ok(forward_reward + healthy_reward - ctrl_cost)
    }

    fn _control_cost(&self, action: &DVector<f64>) -> Result<f64, Error> {
        let squared_actions: f64 = action.iter().map(|x| x * x).sum();
        Ok(self.config.ctrl_cost_weight * squared_actions)
    }

    fn is_healthy(&self) -> Result<bool, Error> {
        let z = self.env.qpos()[1];
        let angle = self.env.qpos()[2];

        let min_z = self.config.healthy_z_range.0;
        let max_z = self.config.healthy_z_range.1;
        let min_angle = self.config.healthy_angle_range.0;
        let max_angle = self.config.healthy_angle_range.1;

        let healthy_z = z > min_z && z < max_z;
        let healthy_angle = angle > min_angle && angle < max_angle;

        Ok(healthy_z && healthy_angle)
    }

    fn _get_reset_info(&self) -> Result<HashMap<String, f64>, Error> {
        let mut info = HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert(
            "z_distance_from_origin".to_string(),
            self.env.qpos()[1] - self.init_qpos[1],
        );
        Ok(info)
    }
}

type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoWalker2dEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.env.reset_to_initial()?;

        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };

        let noise_low = -self.config.reset_noise_scale;
        let noise_high = self.config.reset_noise_scale;

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
        let info = self._get_reset_info()?;

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        let curr_state = self.state()?;

        let x_position_before = self.env.qpos()[0];

        self.env.do_simulation(action.as_slice())?;

        let x_position_after = self.env.qpos()[0];

        let dt = self.env.dt();
        let x_velocity = (x_position_after - x_position_before) / dt;

        let next_state = self._get_obs()?;

        let reward = self._get_rew(x_velocity, &action)?;

        let terminated = (!self.is_healthy()?) && self.config.terminate_when_unhealthy;
        let truncated = self.env.time() > 1000.0; // Common truncation condition

        let mut info = HashMap::new();
        info.insert("x_position".to_string(), x_position_after);
        info.insert(
            "z_distance_from_origin".to_string(),
            self.env.qpos()[1] - self.init_qpos[1],
        );
        info.insert("x_velocity".to_string(), x_velocity);

        let ctrl_cost = self._control_cost(&action)?;
        info.insert(
            "reward_forward".to_string(),
            x_velocity * self.config.forward_reward_weight,
        );
        info.insert("reward_ctrl".to_string(), -ctrl_cost);
        info.insert(
            "reward_survive".to_string(),
            if self.is_healthy()? {
                self.config.healthy_reward
            } else {
                0.0
            },
        );

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal, 0,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok((!self.is_healthy()?) && self.config.terminate_when_unhealthy)
    }

    fn is_truncated(&self) -> bool {
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}
