use super::config::InvertedPendulumConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use crate::env::environment::{Environment, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoInvertedPendulumEnv {
    pub env: MjEnv,
    pub config: InvertedPendulumConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
}

impl MujocoInvertedPendulumEnv {
    pub fn new(config: Option<InvertedPendulumConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(&config.xml_file, config.frame_skip)?;
        Ok(Self {
            init_qpos: env.init_qpos().into(),
            init_qvel: env.init_qvel().into(),
            config,
            env,
        })
    }

    // Helper methods
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();
        observation.extend_from_slice(self.env.qpos());
        observation.extend_from_slice(self.env.qvel());

        Ok(DVector::from_vec(observation))
    }

    fn _calculate_termination(&self, observation: &DVector<f64>) -> Result<bool, Error> {
        // Check if all values are finite and if the pole angle is within limits
        let is_finite = observation.iter().all(|&x| x.is_finite());
        let angle = if observation.len() > 1 {
            observation[1]
        } else {
            0.0
        };
        let is_angle_ok = angle.abs() <= 0.2;

        Ok(!is_finite || !is_angle_ok)
    }

    fn _calculate_reward(&self, terminated: bool) -> Result<f64, Error> {
        if terminated {
            Ok(0.0)
        } else {
            Ok(1.0)
        }
    }

    fn _generate_info(&self, reward: f64) -> Result<HashMap<String, f64>, Error> {
        let mut info = HashMap::new();
        info.insert("reward_survive".to_string(), reward);
        Ok(info)
    }

    fn _get_reset_info(&self) -> Result<HashMap<String, f64>, Error> {
        Ok(HashMap::new())
    }
}

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = HashMap<String, f64>;

impl Environment for MujocoInvertedPendulumEnv {
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

        // Get new observation
        let next_state = self._get_obs()?;

        // Check termination condition
        let terminated = self._calculate_termination(&next_state)?;
        let reward = self._calculate_reward(terminated)?;

        // For inverted pendulum, truncation is typically not based on time but rather on termination
        let truncated = false; // Inverted pendulum doesn't typically have time truncation

        // Create info dict
        let info = self._generate_info(reward)?;

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        let observation = self._get_obs()?;
        Ok(self._calculate_termination(&observation)?)
    }

    fn is_truncated(&self) -> bool {
        false // No time truncation for inverted pendulum
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}
