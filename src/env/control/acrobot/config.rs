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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gravity(mut self, g: f64) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_default_values() {
        let config = AcrobotConfig::default();
        assert_eq!(config.g, 9.8);
        assert_eq!(config.dt, 0.2);
        assert_eq!(config.link_length_1, 1.0);
        assert_eq!(config.link_length_2, 1.0);
        assert_eq!(config.link_mass_1, 1.0);
        assert_eq!(config.link_mass_2, 1.0);
        assert_eq!(config.link_com_pos_1, 0.5);
        assert_eq!(config.link_com_pos_2, 0.5);
        assert_eq!(config.link_moi, 1.0);
        assert_eq!(config.max_vel_1, 4.0 * PI);
        assert_eq!(config.max_vel_2, 9.0 * PI);
        assert_eq!(config.torque_noise_max, 0.0);
        assert_eq!(config.dynamics_mode, DynamicsMode::Book);
        assert!(!config.continuous);
        assert_eq!(config.max_t, 500);
    }

    #[test]
    fn test_new() {
        let config = AcrobotConfig::new();
        assert_eq!(config, AcrobotConfig::default());
    }

    #[test]
    fn test_with_gravity() {
        let config = AcrobotConfig::new().with_gravity(10.0);
        assert_eq!(config.g, 10.0);
    }

    #[test]
    fn test_with_dt() {
        let config = AcrobotConfig::new().with_dt(0.1);
        assert_eq!(config.dt, 0.1);
    }

    #[test]
    fn test_with_link_lengths() {
        let config = AcrobotConfig::new().with_link_lengths(2.0, 3.0);
        assert_eq!(config.link_length_1, 2.0);
        assert_eq!(config.link_length_2, 3.0);
    }

    #[test]
    fn test_with_link_masses() {
        let config = AcrobotConfig::new().with_link_masses(5.0, 6.0);
        assert_eq!(config.link_mass_1, 5.0);
        assert_eq!(config.link_mass_2, 6.0);
    }

    #[test]
    fn test_with_link_com_positions() {
        let config = AcrobotConfig::new().with_link_com_positions(0.1, 0.2);
        assert_eq!(config.link_com_pos_1, 0.1);
        assert_eq!(config.link_com_pos_2, 0.2);
    }

    #[test]
    fn test_with_link_moi() {
        let config = AcrobotConfig::new().with_link_moi(2.5);
        assert_eq!(config.link_moi, 2.5);
    }

    #[test]
    fn test_with_max_velocities() {
        let config = AcrobotConfig::new().with_max_velocities(10.0, 20.0);
        assert_eq!(config.max_vel_1, 10.0);
        assert_eq!(config.max_vel_2, 20.0);
    }

    #[test]
    fn test_with_torque_noise() {
        let config = AcrobotConfig::new().with_torque_noise(0.5);
        assert_eq!(config.torque_noise_max, 0.5);

        let config_neg = AcrobotConfig::new().with_torque_noise(-1.0);
        assert_eq!(config_neg.torque_noise_max, 1.0);
    }

    #[test]
    fn test_dynamics_mode_switching() {
        let config = AcrobotConfig::new().use_nips_dynamics();
        assert_eq!(config.dynamics_mode, DynamicsMode::Nips);

        let config = config.use_book_dynamics();
        assert_eq!(config.dynamics_mode, DynamicsMode::Book);
    }

    #[test]
    fn test_action_space_type_switching() {
        let config = AcrobotConfig::new().with_continuous_action();
        assert!(config.continuous);

        let config = config.with_discrete_action();
        assert!(!config.continuous);
    }

    #[test]
    fn test_with_max_steps() {
        let config = AcrobotConfig::new().with_max_steps(1000);
        assert_eq!(config.max_t, 1000);
    }

    #[test]
    fn test_builder_chaining() {
        let config = AcrobotConfig::new()
            .with_gravity(1.62)
            .use_nips_dynamics()
            .with_continuous_action()
            .with_link_lengths(0.5, 0.5);

        assert_eq!(config.g, 1.62);
        assert_eq!(config.dynamics_mode, DynamicsMode::Nips);
        assert_eq!(config.continuous, true);
        assert_eq!(config.link_length_1, 0.5);
        assert_eq!(config.link_length_2, 0.5);
        assert_eq!(config.dt, 0.2);
    }
}
