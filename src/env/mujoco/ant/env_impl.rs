use nalgebra::DVector;
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::env::{environment::{Environment, Experience, Terminal, Error}, mujoco::mjenv::MjEnv};
use super::{env::MujocoAntEnv, config::AntConfig};

// Define type aliases for clarity
type Action = DVector<f64>;
type State = DVector<f64>;
type Info = std::collections::HashMap<String, f64>;

impl Environment for MujocoAntEnv {
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
            qvel[i] += self.config.reset_noise_scale * rng.random::<f64>() * 2.0 - self.config.reset_noise_scale; // Standard normal would be more complex
        }
        
        self.env.set_state(&qpos, &qvel)?;
        
        let observation = self._get_observation()?;
        let info = self._get_reset_info()?;
        
        Ok((observation, info))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        // Store current state
        let curr_state = self.state()?;
        
        // Get position before simulation
        let xy_pos_before = self._get_body_xpos(self.config.main_body)?;
        
        // Apply action and simulate
        self.env.do_simulation(action.as_slice())?;
        
        // Get position after simulation
        let xy_pos_after = self._get_body_xpos(self.config.main_body)?;
        
        // Calculate velocities
        let dt = self.env.dt();
        let xy_velocity = (xy_pos_after - xy_pos_before) / dt;
        let x_velocity = xy_velocity[0];
        
        // Get new observation
        let next_state = self._get_observation()?;
        
        // Calculate reward
        let (reward, reward_info) = self._calculate_reward(x_velocity, &action)?;
        
        // Determine termination
        let terminated = (!self.is_healthy()?) && self.config.terminate_when_unhealthy;
        let truncated = self.env.time() > 1000.0; // Common truncation condition
        
        // Create info dict
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert("distance_from_origin".to_string(), 
                   (self.env.qpos()[0].powi(2) + self.env.qpos()[1].powi(2)).sqrt());
        info.insert("x_velocity".to_string(), x_velocity);
        info.insert("y_velocity".to_string(), xy_velocity[1]);
        info.extend(reward_info);
        
        let terminal = Terminal::from_flags(terminated, truncated);
        
        Ok(Experience::new(
            curr_state,
            reward,
            action,
            next_state,
            info,
            terminal,
            0, // Step counter would need to be tracked separately
        ))
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok((!self.is_healthy()?) && self.config.terminate_when_unhealthy)
    }

    fn is_truncated(&self) -> bool {
        // For now, using a simple time-based truncation
        self.env.time() > 1000.0
    }

    fn state(&self) -> Result<Self::State, Error> {
        self._get_observation()
    }
}

impl MujocoAntEnv {
    // Helper methods
    fn _get_observation(&self) -> Result<DVector<f64>, Error> {
        let mut observation = Vec::new();
        
        let mut position = self.env.qpos().to_vec();
        let velocity = self.env.qvel().to_vec();
        
        if self.config.exclude_current_positions_from_observation {
            // Skip first 2 elements (x, y position)
            position = position[2..].to_vec();
        }
        
        observation.extend_from_slice(&position);
        observation.extend_from_slice(&velocity);
        
        if self.config.include_cfrc_ext_in_observation {
            // Add clipped contact forces (excluding first body which is usually world)
            let contact_forces = self._get_contact_forces();
            observation.extend_from_slice(&contact_forces[3..]); // Skip first 3 values (first body)
        }
        
        Ok(DVector::from_vec(observation))
    }
    
    fn _get_contact_forces(&self) -> Vec<f64> {
        let mut forces = Vec::new();
        let cfrc_ext = self.env.cfrc_ext();
        
        for body_forces in cfrc_ext.iter() {
            let min_val = self.config.contact_force_range.0;
            let max_val = self.config.contact_force_range.1;
            for &force in body_forces.iter() {
                forces.push(force.clamp(min_val, max_val));
            }
        }
        
        forces
    }
    
    fn _calculate_reward(&self, x_velocity: f64, action: &DVector<f64>) -> Result<(f64, std::collections::HashMap<String, f64>), Error> {
        let forward_reward = x_velocity * self.config.forward_reward_weight;
        let healthy_reward = self.healthy_reward()?;
        let ctrl_cost = self._control_cost(action)?;
        let contact_cost = self._contact_cost()?;
        
        let total_reward = forward_reward + healthy_reward - ctrl_cost - contact_cost;
        
        let mut reward_info = std::collections::HashMap::new();
        reward_info.insert("reward_forward".to_string(), forward_reward);
        reward_info.insert("reward_ctrl".to_string(), -ctrl_cost);
        reward_info.insert("reward_contact".to_string(), -contact_cost);
        reward_info.insert("reward_survive".to_string(), healthy_reward);
        
        Ok((total_reward, reward_info))
    }
    
    fn _control_cost(&self, action: &DVector<f64>) -> Result<f64, Error> {
        let squared_actions: f64 = action.iter().map(|x| x * x).sum();
        Ok(self.config.ctrl_cost_weight * squared_actions)
    }
    
    fn _contact_cost(&self) -> Result<f64, Error> {
        let contact_forces = self._get_contact_forces();
        let squared_forces: f64 = contact_forces.iter().map(|x| x * x).sum();
        Ok(self.config.contact_cost_weight * squared_forces)
    }
    
    fn is_healthy(&self) -> Result<bool, Error> {
        let state = self.env.state_vector();
        let min_z = self.config.healthy_z_range.0;
        let max_z = self.config.healthy_z_range.1;
        
        let is_finite = state.iter().all(|&x| x.is_finite());
        let z_pos = state[2]; // Third element is z position
        
        Ok(is_finite && z_pos >= min_z && z_pos <= max_z)
    }
    
    fn healthy_reward(&self) -> Result<f64, Error> {
        if self.is_healthy()? {
            Ok(self.config.healthy_reward)
        } else {
            Ok(0.0)
        }
    }
    
    fn _get_body_xpos(&self, body_id: u32) -> Result<nalgebra::Vector3<f64>, Error> {
        // Get body position from model, with fallback to xipos
        if (body_id as usize) < self.env.xipos().len() {
            let pos = self.env.xipos()[body_id as usize];
            Ok(nalgebra::Vector3::new(pos[0], pos[1], pos[2]))
        } else {
            // If body_id is out of bounds, return an error
            Err(Error::InvalidStateDimension { 
                field: "body_id", 
                expected: self.env.nbody(), 
                got: body_id as usize 
            })
        }
    }
    
    fn _get_reset_info(&self) -> Result<Info, Error> {
        let mut info = std::collections::HashMap::new();
        info.insert("x_position".to_string(), self.env.qpos()[0]);
        info.insert("y_position".to_string(), self.env.qpos()[1]);
        info.insert("distance_from_origin".to_string(), 
                   (self.env.qpos()[0].powi(2) + self.env.qpos()[1].powi(2)).sqrt());
        Ok(info)
    }
}