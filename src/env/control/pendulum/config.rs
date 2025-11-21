use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendulumConfig {
    pub n: u32,
    pub g: f64,
    pub f: f64,
    pub m: f64,
    pub l: f64,
    pub x0: f64,
    pub y0: f64,
    pub dt: f64,
    pub max_t: u32,
    pub max_v: f64,
    pub max_tau: f64,
    pub continuous: bool,
}

impl Default for PendulumConfig {
    fn default() -> Self {
        Self {
            f: 2.,
            g: 10.,
            m: 1.,
            l: 1.,
            x0: PI,
            y0: 1.,
            dt: 5e-2,
            max_v: 8.,
            max_tau: 2.,
            max_t: 200,
            n: 2,
            continuous: true,
        }
    }
}

impl PendulumConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gravity(mut self, g: f64) -> Self {
        self.g = g;
        self
    }

    pub fn with_force(mut self, f: f64) -> Self {
        self.f = f;
        self
    }

    pub fn with_mass(mut self, m: f64) -> Self {
        self.m = m;
        self
    }

    pub fn with_length(mut self, l: f64) -> Self {
        self.l = l;
        self
    }

    pub fn with_initial_angle(mut self, x0: f64) -> Self {
        self.x0 = x0;
        self
    }

    pub fn with_initial_velocity(mut self, y0: f64) -> Self {
        self.y0 = y0;
        self
    }

    pub fn with_timestep(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    pub fn with_max_steps(mut self, max_t: u32) -> Self {
        self.max_t = max_t;
        self
    }

    pub fn with_max_velocity(mut self, max_v: f64) -> Self {
        self.max_v = max_v;
        self
    }

    pub fn with_max_torque(mut self, max_tau: f64) -> Self {
        self.max_tau = max_tau;
        self
    }

    pub fn with_discrete_action(mut self, n: u32) -> Self {
        self.n = n;
        self.continuous = false;
        self
    }

    pub fn with_continuous_action(mut self) -> Self {
        self.continuous = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_default_values() {
        let config = PendulumConfig::default();

        assert_eq!(config.f, 2.0);
        assert_eq!(config.g, 10.0);
        assert_eq!(config.m, 1.0);
        assert_eq!(config.l, 1.0);
        assert_eq!(config.x0, PI);
        assert_eq!(config.y0, 1.0);
        assert_eq!(config.dt, 0.05);
        assert_eq!(config.max_v, 8.0);
        assert_eq!(config.max_tau, 2.0);
        assert_eq!(config.max_t, 200);
        assert_eq!(config.n, 2);
        assert!(config.continuous);
    }

    #[test]
    fn test_new() {
        let config = PendulumConfig::new();
        assert_eq!(config, PendulumConfig::default());
    }

    #[test]
    fn test_builder_gravity() {
        let config = PendulumConfig::new().with_gravity(9.81);
        assert_eq!(config.g, 9.81);
    }

    #[test]
    fn test_builder_force() {
        let config = PendulumConfig::new().with_force(5.0);
        assert_eq!(config.f, 5.0);
    }

    #[test]
    fn test_builder_mass() {
        let config = PendulumConfig::new().with_mass(2.5);
        assert_eq!(config.m, 2.5);
    }

    #[test]
    fn test_builder_length() {
        let config = PendulumConfig::new().with_length(1.5);
        assert_eq!(config.l, 1.5);
    }

    #[test]
    fn test_builder_initial_angle() {
        let config = PendulumConfig::new().with_initial_angle(0.5);
        assert_eq!(config.x0, 0.5);
    }

    #[test]
    fn test_builder_initial_velocity() {
        let config = PendulumConfig::new().with_initial_velocity(0.1);
        assert_eq!(config.y0, 0.1);
    }

    #[test]
    fn test_builder_timestep() {
        let config = PendulumConfig::new().with_timestep(0.01);
        assert_eq!(config.dt, 0.01);
    }

    #[test]
    fn test_builder_max_steps() {
        let config = PendulumConfig::new().with_max_steps(500);
        assert_eq!(config.max_t, 500);
    }

    #[test]
    fn test_builder_max_velocity() {
        let config = PendulumConfig::new().with_max_velocity(10.0);
        assert_eq!(config.max_v, 10.0);
    }

    #[test]
    fn test_builder_max_torque() {
        let config = PendulumConfig::new().with_max_torque(3.0);
        assert_eq!(config.max_tau, 3.0);
    }

    #[test]
    fn test_builder_discrete_action() {
        let config = PendulumConfig::new().with_discrete_action(5);
        assert_eq!(config.n, 5);
        assert!(!config.continuous);
    }

    #[test]
    fn test_builder_continuous_action() {
        let config = PendulumConfig::new()
            .with_discrete_action(5)
            .with_continuous_action();

        assert!(config.continuous);
        assert_eq!(config.n, 5);
    }

    #[test]
    fn test_builder_chaining() {
        let config = PendulumConfig::new()
            .with_gravity(9.8)
            .with_mass(2.0)
            .with_length(0.5)
            .with_discrete_action(3);

        assert_eq!(config.g, 9.8);
        assert_eq!(config.m, 2.0);
        assert_eq!(config.l, 0.5);
        assert_eq!(config.n, 3);
        assert!(!config.continuous);

        assert_eq!(config.max_v, 8.0);
    }

    #[test]
    fn test_clone_and_copy() {
        let config1 = PendulumConfig::new();
        let config2 = config1;
        let config3 = config1.clone();

        assert_eq!(config1, config2);
        assert_eq!(config1, config3);
    }

    #[test]
    fn test_debug_implementation() {
        let config = PendulumConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("PendulumConfig"));
        assert!(debug_str.contains("g: 10.0"));
    }
}
