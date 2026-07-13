use super::config::InvertedDoublePendulumConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use crate::env::environment::{Environment, Experience, Terminal};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

pub struct MujocoInvertedDoublePendulumEnv {
    pub env: MjEnv,
    pub config: InvertedDoublePendulumConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
    steps: usize,
}

impl MujocoInvertedDoublePendulumEnv {
    pub fn new(config: Option<InvertedDoublePendulumConfig>) -> Result<Self, Error> {
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

    /// Matches Gymnasium's InvertedDoublePendulum-v4 exactly: cart x
    /// position, sin/cos of the two pole angles (not the raw angles),
    /// velocity-clamped qvel, and the constraint force on the last DOF
    /// (hinge2) clamped the same way.
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let qpos = self.env.qpos();
        let qvel = self.env.qvel();
        let qfrc_constraint = self.env.qfrc_constraint();

        let mut observation = Vec::with_capacity(9);
        observation.push(qpos[0]);
        observation.push(qpos[1].sin());
        observation.push(qpos[2].sin());
        observation.push(qpos[1].cos());
        observation.push(qpos[2].cos());
        observation.extend(qvel.iter().map(|v| v.clamp(-10.0, 10.0)));
        observation.push(
            qfrc_constraint
                .last()
                .copied()
                .unwrap_or(0.0)
                .clamp(-10.0, 10.0),
        );

        Ok(DVector::from_vec(observation))
    }

    /// World position of the "tip" site (the free end of the second pole).
    fn _tip_pos(&self) -> [f64; 3] {
        self.env.site_xpos()[0]
    }

    /// Matches Gymnasium: the pendulum is considered fallen once the tip's
    /// height drops to or below 1.0 (world z, since this model is built in
    /// the x-z plane).
    fn _has_fallen(&self) -> Result<bool, Error> {
        let tip_height = self._tip_pos()[2];
        Ok(tip_height <= 1.0)
    }

    /// reward = alive_bonus - dist_penalty - vel_penalty, matching
    /// Gymnasium's InvertedDoublePendulum-v4 reward exactly. dist_penalty
    /// pulls the tip toward (x=0, z=2) (upright, centered); vel_penalty
    /// damps both pole angular velocities.
    fn _calculate_reward(&self) -> f64 {
        let tip = self._tip_pos();
        let dist_penalty = 0.01 * tip[0].powi(2) + (tip[2] - 2.0).powi(2);

        let qvel = self.env.qvel();
        let v1 = qvel[1];
        let v2 = qvel[2];
        let vel_penalty = 1e-3 * v1.powi(2) + 5e-3 * v2.powi(2);

        self.config.healthy_reward - dist_penalty - vel_penalty
    }

    fn _get_info(&self) -> Result<HashMap<String, f64>, Error> {
        let mut info = HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("x_velocity".to_string(), self.env.qvel()[0]);
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

impl Environment for MujocoInvertedDoublePendulumEnv {
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

        let noise_low = -self.config.reset_noise_scale;
        let noise_high = self.config.reset_noise_scale;

        // Add noise to positions
        let mut qpos = self.init_qpos.clone();
        if noise_high > noise_low {
            for i in 0..qpos.len() {
                qpos[i] += rng.random_range(noise_low..noise_high);
            }
        }

        // Add noise to velocities
        let mut qvel = self.init_qvel.clone();
        if noise_high > noise_low {
            for i in 0..qvel.len() {
                qvel[i] += rng.random_range(noise_low..noise_high);
            }
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

        let reward = self._calculate_reward();

        let terminated = self._has_fallen()?;
        let truncated = self.steps >= self.config.max_steps;

        // Create info dict
        let info = self._get_info()?;

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
        Ok(self._has_fallen()?)
    }

    fn is_truncated(&self) -> bool {
        self.steps >= self.config.max_steps
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_with_zero_noise_scale_does_not_panic() {
        let config = InvertedDoublePendulumConfig {
            reset_noise_scale: 0.0,
            ..InvertedDoublePendulumConfig::default()
        };
        let mut env = MujocoInvertedDoublePendulumEnv::new(Some(config)).unwrap();
        assert!(env.reset(None).is_ok());
    }

    #[test]
    fn test_truncation_at_max_episode_steps() {
        let config = InvertedDoublePendulumConfig {
            max_steps: 5,
            ..InvertedDoublePendulumConfig::default()
        };
        let mut env = MujocoInvertedDoublePendulumEnv::new(Some(config)).unwrap();
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
    fn test_observation_dimension_and_layout() {
        let mut env = MujocoInvertedDoublePendulumEnv::new(None).unwrap();
        env.reset(Some(0)).unwrap();

        let obs = env.state().unwrap();
        assert_eq!(obs.len(), 9);
        assert_eq!(obs.len(), env.config.observation_shape.0);

        let qpos = env.env.qpos().to_vec();
        assert!((obs[0] - qpos[0]).abs() < 1e-9);
        assert!((obs[1] - qpos[1].sin()).abs() < 1e-9);
        assert!((obs[2] - qpos[2].sin()).abs() < 1e-9);
        assert!((obs[3] - qpos[1].cos()).abs() < 1e-9);
        assert!((obs[4] - qpos[2].cos()).abs() < 1e-9);
    }

    #[test]
    fn test_reward_matches_tip_based_formula() {
        let mut env = MujocoInvertedDoublePendulumEnv::new(None).unwrap();
        env.reset(Some(0)).unwrap();

        let action = DVector::zeros(env.env.nu());
        let exp = env.step(action).unwrap();

        let tip = env._tip_pos();
        let dist_penalty = 0.01 * tip[0].powi(2) + (tip[2] - 2.0).powi(2);
        let qvel = env.env.qvel();
        let vel_penalty = 1e-3 * qvel[1].powi(2) + 5e-3 * qvel[2].powi(2);
        let expected_reward = env.config.healthy_reward - dist_penalty - vel_penalty;

        assert!((exp.reward - expected_reward).abs() < 1e-9);
    }

    #[test]
    fn test_termination_based_on_tip_height() {
        let mut env = MujocoInvertedDoublePendulumEnv::new(None).unwrap();
        env.reset(Some(0)).unwrap();

        // Freshly reset near-upright: tip should be near max height (z~2), not fallen.
        assert!(!env.is_terminal().unwrap());

        // Force the tip down by rotating both poles hard over; qpos = [cart, theta1, theta2].
        let mut qpos = env.env.qpos().to_vec();
        qpos[1] = std::f64::consts::PI;
        qpos[2] = std::f64::consts::PI;
        let qvel = vec![0.0; env.env.nv()];
        env.env.set_state(&qpos, &qvel).unwrap();

        assert!(env.is_terminal().unwrap());
    }
}
