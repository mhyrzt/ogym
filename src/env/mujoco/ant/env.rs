use super::config::AntConfig;
use crate::env::environment::{Environment, Experience, Terminal};
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoAntEnv {
    pub env: MjEnv,
    pub config: AntConfig,
    init_qpos: Vec<f64>,
    init_qvel: Vec<f64>,
    steps: usize,
}

impl MujocoAntEnv {
    pub fn new(config: Option<AntConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(&config.xml, config.frame_skip)?;

        Ok(Self {
            init_qpos: env.init_qpos().into(),
            init_qvel: env.init_qvel().into(),
            config,
            env,
            steps: 0,
        })
    }

    fn _get_observation(&self) -> Result<DVector<f64>, Error> {
        let qpos = self.env.qpos();
        let qvel = self.env.qvel();

        let start_idx = if self.config.exclude_current_positions_from_observation {
            2
        } else {
            0
        };

        let cfrc_len = if self.config.include_cfrc_ext_in_observation {
            self.env.nbody() * 6
        } else {
            0
        };
        let mut obs_vec = Vec::with_capacity((qpos.len() - start_idx) + qvel.len() + cfrc_len);

        obs_vec.extend_from_slice(&qpos[start_idx..]);
        obs_vec.extend_from_slice(qvel);

        if self.config.include_cfrc_ext_in_observation {
            let contact_forces = self._get_contact_forces();
            obs_vec.extend(contact_forces);
        }

        Ok(DVector::from_vec(obs_vec))
    }

    fn _get_contact_forces(&self) -> impl Iterator<Item = f64> + '_ {
        let (min_val, max_val) = self.config.contact_force_range;
        self.env.cfrc_ext().iter().flat_map(move |body_forces| {
            body_forces.iter().map(move |&f| f.clamp(min_val, max_val))
        })
    }

    fn _calculate_reward(
        &self,
        x_velocity: f64,
        action: &DVector<f64>,
    ) -> Result<(f64, HashMap<String, f64>), Error> {
        let forward_reward = x_velocity * self.config.forward_reward_weight;
        let healthy_reward = if self.is_healthy()? {
            self.config.healthy_reward
        } else {
            0.0
        };

        let ctrl_cost =
            self.config.ctrl_cost_weight * action.iter().map(|x| x.powi(2)).sum::<f64>();

        let contact_cost = self.config.contact_cost_weight
            * self._get_contact_forces().map(|x| x.powi(2)).sum::<f64>();

        let total_reward = forward_reward + healthy_reward - ctrl_cost - contact_cost;

        let mut reward_info = HashMap::new();
        reward_info.insert("reward_forward".to_string(), forward_reward);
        reward_info.insert("reward_ctrl".to_string(), -ctrl_cost);
        reward_info.insert("reward_contact".to_string(), -contact_cost);
        reward_info.insert("reward_survive".to_string(), healthy_reward);

        Ok((total_reward, reward_info))
    }

    fn is_healthy(&self) -> Result<bool, Error> {
        let state = self.env.state_vector();
        let (min_z, max_z) = self.config.healthy_z_range;
        let z_pos = state[2];
        let is_finite = state.iter().all(|x| x.is_finite());
        Ok(is_finite && z_pos >= min_z && z_pos <= max_z)
    }

    fn _get_body_xpos(&self, body_id: u32) -> Result<nalgebra::Vector3<f64>, Error> {
        let xipos = self.env.xipos();
        if (body_id as usize) < xipos.len() {
            let pos = xipos[body_id as usize];
            Ok(nalgebra::Vector3::new(pos[0], pos[1], pos[2]))
        } else {
            Err(Error::InvalidStateDimension {
                field: "body_id",
                expected: xipos.len(),
                got: body_id as usize,
            })
        }
    }
}

type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoAntEnv {
    type Action = Action;
    type State = State;
    type Info = Info;

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.steps = 0;
        self.env.reset_to_initial()?;

        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };

        let scale = self.config.reset_noise_scale;
        let noise_range = -scale..scale;

        let mut qpos = self.init_qpos.clone();
        if scale > 0.0 {
            qpos.iter_mut()
                .for_each(|val| *val += rng.random_range(noise_range.clone()));
        }

        let mut qvel = self.init_qvel.clone();
        qvel.iter_mut().for_each(|val| {
            *val += scale * rng.random::<f64>() * 2.0 - scale;
        });

        self.env.set_state(&qpos, &qvel)?;

        unsafe {
            mujoco_rs_sys::no_render::mj_forward(
                self.env.model().ptr(),
                self.env.state_mut().ptr(),
            );
        }

        let observation = self._get_observation()?;

        let mut info = HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert(
            "distance_from_origin".to_string(),
            self.env.qpos()[0].hypot(self.env.qpos()[1]),
        );

        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.steps += 1;

        let curr_state = self.state()?;
        let pos_before = self.env.qpos().to_vec();

        self.env.do_simulation(action.as_slice())?;

        let pos_after = self.env.qpos();
        let dt = self.env.dt();
        let x_velocity = (pos_after[0] - pos_before[0]) / dt;
        let y_velocity = (pos_after[1] - pos_before[1]) / dt;

        let next_state = self._get_observation()?;
        let (reward, reward_info) = self._calculate_reward(x_velocity, &action)?;

        let is_healthy = self.is_healthy()?;
        let terminated = self.config.terminate_when_unhealthy && !is_healthy;
        let truncated = self.steps >= self.config.max_episode_steps;

        let mut info = HashMap::new();
        info.insert("x_position".to_string(), pos_after[0]);
        info.insert("y_position".to_string(), pos_after[1]);
        info.insert(
            "distance_from_origin".to_string(),
            pos_after[0].hypot(pos_after[1]),
        );
        info.insert("x_velocity".to_string(), x_velocity);
        info.insert("y_velocity".to_string(), y_velocity);
        info.extend(reward_info);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal, 0,
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(self.config.terminate_when_unhealthy && !self.is_healthy()?)
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_episode_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_observation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_XML: &str = r#"
    <mujoco model="ant_test_minimal">
        <compiler angle="radian"/>
        <option timestep="0.01" gravity="0 0 -9.81"/>
        <worldbody>
            <geom type="plane" size="10 10 0.1"/>
            <body name="torso" pos="0 0 0.75">
                <joint name="root" type="free"/>
                <geom type="sphere" size="0.25" mass="1"/>
            </body>
        </worldbody>
        <actuator>
            <motor name="motor_1" joint="root" gear="1"/> 
        </actuator>
    </mujoco>
    "#;

    #[test]
    fn test_builder_methods_compile() {
        let config = AntConfig::new()
            .with_include_cfrc_ext_in_observation(true)
            .with_exclude_current_positions_from_observation(false);

        assert!(config.include_cfrc_ext_in_observation);
        assert!(!config.exclude_current_positions_from_observation);
    }

    #[test]
    fn test_env_with_mock_xml() {
        let config = AntConfig::new()
            .with_xml(TEST_XML)
            .with_include_cfrc_ext_in_observation(false)
            .with_exclude_current_positions_from_observation(false);

        let env = MujocoAntEnv::new(Some(config)).unwrap();

        assert_eq!(env.env.nq(), 7);
        assert_eq!(env.env.nv(), 6);
    }

    #[test]
    fn test_env_with_real_default_xml() {
        let config = AntConfig::default(); // Load real model
        let env_res = MujocoAntEnv::new(Some(config));
        assert!(
            env_res.is_ok(),
            "Should load the default include_str! model successfully"
        );

        if let Ok(env) = env_res {
            assert!(env.env.nq() > 0);
            assert!(env.env.dt() > 0.0);
        }
    }

    #[test]
    fn test_reset_with_zero_noise_scale_does_not_panic() {
        let config = AntConfig::new()
            .with_xml(TEST_XML)
            .with_reset_noise_scale(0.0);
        let mut env = MujocoAntEnv::new(Some(config)).unwrap();
        assert!(env.reset(None).is_ok());
    }

    #[test]
    fn test_step_cycle() {
        let config = AntConfig::new().with_xml(TEST_XML);
        let mut env = MujocoAntEnv::new(Some(config)).unwrap();
        env.reset(None).unwrap();

        let action = DVector::zeros(env.env.nu());
        let exp = env.step(action).unwrap();

        assert!(!exp.next_state.is_empty());
        assert!(exp.reward.is_finite());
    }
}
