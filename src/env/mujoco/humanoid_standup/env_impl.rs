use super::{config::HumanoidStandupConfig, env::MujocoHumanoidStandupEnv};
use crate::env::{
    environment::{Environment, Error, Experience, Terminal},
    mujoco::mjenv::MjEnv,
};
use nalgebra::DVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoHumanoidStandupEnv {
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
        let info = self._get_reset_info()?;

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
        let (reward, reward_info) = self._calculate_reward(&action)?;

        // For humanoid standup, no termination (it's a standup task)
        let terminated = false;
        let truncated = self.env.time() > 1000.0; // Time-based truncation

        // Create info dict
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert(
            "tendon_length".to_string(),
            self.env.ten_length().iter().sum(),
        ); // Sum as a placeholder since it's an array
        info.insert(
            "tendon_velocity".to_string(),
            self.env.ten_velocity().iter().sum(),
        ); // Sum as a placeholder since it's an array
        info.insert(
            "distance_from_origin".to_string(),
            (self.env.qpos()[0].powi(2) + self.env.qpos()[1].powi(2)).sqrt(),
        );
        info.extend(reward_info);

        let terminal = Terminal::from_flags(terminated, truncated);

        Ok(Experience::new(
            curr_state, reward, action, next_state, info, terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        // Humanoid standup doesn't terminate based on health
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        // Time-based truncation
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_obs()
    }
}

impl MujocoHumanoidStandupEnv {
    // Helper methods
    fn _get_obs(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();

        let mut position = self.env.qpos().to_vec();
        let velocity = self.env.qvel().to_vec();

        // Get additional observation components based on config (similar to humanoid)
        let com_inertia = if self.config.include_cinert_in_observation {
            let cinert = self.env.cinert();
            let mut cinert_flat = Vec::new();
            for body_cinert in cinert.iter().skip(1) {
                // Skip first body
                for &val in body_cinert.iter() {
                    cinert_flat.push(val);
                }
            }
            cinert_flat
        } else {
            Vec::new()
        };

        let com_velocity = if self.config.include_cvel_in_observation {
            let cvel = self.env.cvel();
            let mut cvel_flat = Vec::new();
            for body_cvel in cvel.iter().skip(1) {
                // Skip first body
                for &val in body_cvel.iter() {
                    cvel_flat.push(val);
                }
            }
            cvel_flat
        } else {
            Vec::new()
        };

        let actuator_forces = if self.config.include_qfrc_actuator_in_observation {
            let qfrc = self.env.qfrc_actuator();
            qfrc[6..].to_vec() // Skip first 6 values
        } else {
            Vec::new()
        };

        let external_contact_forces = if self.config.include_cfrc_ext_in_observation {
            let cfrc_ext = self.env.cfrc_ext();
            let mut cfrc_ext_flat = Vec::new();
            for body_cfrc in cfrc_ext.iter().skip(1) {
                // Skip first body
                for &val in body_cfrc.iter() {
                    cfrc_ext_flat.push(val);
                }
            }
            cfrc_ext_flat
        } else {
            Vec::new()
        };

        if self.config.exclude_current_positions_from_observation {
            // Skip first 2 elements (x, y position)
            position = position[2..].to_vec();
        }

        observation.extend_from_slice(&position);
        observation.extend_from_slice(&velocity);
        observation.extend_from_slice(&com_inertia);
        observation.extend_from_slice(&com_velocity);
        observation.extend_from_slice(&actuator_forces);
        observation.extend_from_slice(&external_contact_forces);

        Ok(DVector::from_vec(observation))
    }

    fn _calculate_reward(
        &self,
        action: &DVector<f64>,
    ) -> Result<(f64, std::collections::HashMap<String, f64>), Error> {
        // Upright reward based on z position of torso (should be maximized)
        let upright_reward = self.config.uph_cost_weight * self.env.qpos()[2]; // z position of torso/root

        let ctrl_cost = self._control_cost(action)?;
        let impact_cost = self._impact_cost()?;

        let total_reward = upright_reward - ctrl_cost - impact_cost;

        let mut reward_info = std::collections::HashMap::new();
        reward_info.insert("reward_upright".to_string(), upright_reward);
        reward_info.insert("reward_ctrl".to_string(), -ctrl_cost);
        reward_info.insert("reward_impact".to_string(), -impact_cost);

        Ok((total_reward, reward_info))
    }

    fn _control_cost(&self, _action: &DVector<f64>) -> Result<f64, Error> {
        let squared_ctrl: f64 = self.env.ctrl().iter().map(|x| x * x).sum();
        Ok(self.config.ctrl_cost_weight * squared_ctrl)
    }

    fn _impact_cost(&self) -> Result<f64, Error> {
        // Impact cost based on external forces
        let contact_forces = self.env.cfrc_ext();
        let mut total_force = 0.0;
        for body_force in contact_forces.iter() {
            for &f in body_force.iter() {
                total_force += f * f;
            }
        }
        let impact_cost = self.config.impact_cost_weight * total_force;
        let clamped_cost = impact_cost.clamp(
            self.config.impact_cost_range.0,
            self.config.impact_cost_range.1,
        );

        Ok(clamped_cost)
    }

    fn _get_reset_info(&self) -> Result<Info, Error> {
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert(
            "tendon_length".to_string(),
            self.env.ten_length().iter().sum(),
        ); // Sum as a placeholder since it's an array
        info.insert(
            "tendon_velocity".to_string(),
            self.env.ten_velocity().iter().sum(),
        ); // Sum as a placeholder since it's an array
        info.insert(
            "distance_from_origin".to_string(),
            (self.env.qpos()[0].powi(2) + self.env.qpos()[1].powi(2)).sqrt(),
        );
        Ok(info)
    }
}
