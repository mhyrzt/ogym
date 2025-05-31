use std::f64::consts::PI;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DynamicsMode {
    Book,
    Nips,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcrobotConfig {
    pub g: f64,
    pub dt: f64,
    pub link_length_1: f64,
    pub link_length_2: f64,
    pub link_mass_1: f64,
    pub link_mass_2: f64,
    pub link_com_pos_1: f64,
    pub link_com_pos_2: f64,
    pub link_moi: f64,
    pub max_vel_1: f64,
    pub max_vel_2: f64,
    pub torque_noise_max: f64,
    pub dynamics_mode: DynamicsMode,
    pub continuous: bool,
    pub max_t: u32,
}

impl Default for AcrobotConfig {
    fn default() -> Self {
        Self {
            g: 9.8,
            dt: 0.2,
            link_length_1: 1.0,
            link_length_2: 1.0,
            link_mass_1: 1.0,
            link_mass_2: 1.0,
            link_com_pos_1: 0.5,
            link_com_pos_2: 0.5,
            link_moi: 1.0,
            max_vel_1: 4.0 * PI,
            max_vel_2: 9.0 * PI,
            torque_noise_max: 0.0,
            dynamics_mode: DynamicsMode::Book,
            continuous: false,
            max_t: 500,
        }
    }
}

impl AcrobotConfig {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn wit_gravity(mut self, g: f64) -> Self {
        self.g = g;
        self
    }

    pub fn with_dt(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    pub fn with_link_lengths(mut self, l1: f64, l2: f64) -> Self {
        self.link_length_1 = l1;
        self.link_length_2 = l2;
        self
    }

    pub fn with_link_masses(mut self, m1: f64, m2: f64) -> Self {
        self.link_mass_1 = m1;
        self.link_mass_2 = m2;
        self
    }

    pub fn with_link_com_positions(mut self, c1: f64, c2: f64) -> Self {
        self.link_com_pos_1 = c1;
        self.link_com_pos_2 = c2;
        self
    }

    pub fn with_link_moi(mut self, moi: f64) -> Self {
        self.link_moi = moi;
        self
    }

    pub fn with_max_velocities(mut self, v1: f64, v2: f64) -> Self {
        self.max_vel_1 = v1;
        self.max_vel_2 = v2;
        self
    }

    pub fn with_torque_noise(mut self, noise: f64) -> Self {
        self.torque_noise_max = noise.abs();
        self
    }

    pub fn use_book_dynamics(mut self) -> Self {
        self.dynamics_mode = DynamicsMode::Book;
        self
    }

    pub fn use_nips_dynamics(mut self) -> Self {
        self.dynamics_mode = DynamicsMode::Nips;
        self
    }

    pub fn with_continuous_action(mut self) -> Self {
        self.continuous = true;
        self
    }

    pub fn with_discrete_action(mut self) -> Self {
        self.continuous = false;
        self
    }

    pub fn with_max_steps(mut self, max_t: u32) -> Self {
        self.max_t = max_t;
        self
    }
}
