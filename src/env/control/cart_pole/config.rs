#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KinematicsIntegrator {
    Euler,
    SemiImplicitEuler,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartPoleConfig {
    pub g: f64,
    pub f: f64,
    pub l: f64,
    pub mc: f64,
    pub mp: f64,
    pub tau: f64,
    pub x_max: f64,
    pub theta_max: f64,
    pub t_max: u32,
    pub continuous: bool,
    pub integrator: KinematicsIntegrator,
}

impl Default for CartPoleConfig {
    fn default() -> Self {
        Self {
            g: 9.8,
            f: 10.0,
            l: 0.5,
            mc: 1.0,
            mp: 0.1,
            tau: 0.02,
            x_max: 2.4,
            theta_max: (12.0f64).to_radians(),
            t_max: 500,
            continuous: false,
            integrator: KinematicsIntegrator::Euler,
        }
    }
}

impl CartPoleConfig {
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

    pub fn with_pole_length(mut self, l: f64) -> Self {
        self.l = l;
        self
    }

    pub fn with_cart_mass(mut self, mc: f64) -> Self {
        self.mc = mc;
        self
    }

    pub fn with_pole_mass(mut self, mp: f64) -> Self {
        self.mp = mp;
        self
    }

    pub fn with_timestep(mut self, tau: f64) -> Self {
        self.tau = tau;
        self
    }

    pub fn with_x_max(mut self, x_max: f64) -> Self {
        self.x_max = x_max;
        self
    }

    pub fn with_theta_max_degrees(mut self, deg: f64) -> Self {
        self.theta_max = deg.to_radians();
        self
    }

    pub fn with_theta_max(mut self, radians: f64) -> Self {
        self.theta_max = radians;
        self
    }

    pub fn with_t_max(mut self, t_max: u32) -> Self {
        self.t_max = t_max;
        self
    }

    pub fn with_euler_integrator(mut self) -> Self {
        self.integrator = KinematicsIntegrator::Euler;
        self
    }

    pub fn with_semi_implicit_euler_integrator(mut self) -> Self {
        self.integrator = KinematicsIntegrator::SemiImplicitEuler;
        self
    }

    pub fn with_discrete_action(mut self) -> Self {
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

    #[test]
    fn test_default_config() {
        let config = CartPoleConfig::default();
        assert_eq!(config.g, 9.8);
        assert_eq!(config.f, 10.0);
        assert_eq!(config.l, 0.5);
        assert_eq!(config.mc, 1.0);
        assert_eq!(config.mp, 0.1);
        assert_eq!(config.tau, 0.02);
        assert_eq!(config.x_max, 2.4);
        assert_eq!(config.theta_max, 12.0f64.to_radians());
        assert_eq!(config.t_max, 500);
        assert_eq!(config.continuous, false);
        assert_eq!(config.integrator, KinematicsIntegrator::Euler);
    }

    #[test]
    fn test_new_config() {
        let config = CartPoleConfig::new();
        let default_config = CartPoleConfig::default();
        assert_eq!(config, default_config);
    }

    #[test]
    fn test_builder_methods_numeric() {
        let config = CartPoleConfig::new()
            .with_gravity(10.0)
            .with_force(15.0)
            .with_pole_length(0.8)
            .with_cart_mass(1.5)
            .with_pole_mass(0.2)
            .with_timestep(0.05)
            .with_x_max(3.0)
            .with_t_max(1000);

        assert_eq!(config.g, 10.0);
        assert_eq!(config.f, 15.0);
        assert_eq!(config.l, 0.8);
        assert_eq!(config.mc, 1.5);
        assert_eq!(config.mp, 0.2);
        assert_eq!(config.tau, 0.05);
        assert_eq!(config.x_max, 3.0);
        assert_eq!(config.t_max, 1000);
    }

    #[test]
    fn test_theta_configuration() {
        let config = CartPoleConfig::new().with_theta_max_degrees(24.0);
        assert_eq!(config.theta_max, 24.0f64.to_radians());

        let config = config.with_theta_max(0.5);
        assert_eq!(config.theta_max, 0.5);
    }

    #[test]
    fn test_integrator_toggle() {
        let config = CartPoleConfig::new().with_semi_implicit_euler_integrator();
        assert_eq!(config.integrator, KinematicsIntegrator::SemiImplicitEuler);

        let config = config.with_euler_integrator();
        assert_eq!(config.integrator, KinematicsIntegrator::Euler);
    }

    #[test]
    fn test_action_toggle() {
        let config = CartPoleConfig::new().with_continuous_action();
        assert!(config.continuous);

        let config = config.with_discrete_action();
        assert!(!config.continuous);
    }

    #[test]
    fn test_clone_and_debug() {
        let config = CartPoleConfig::default();
        let cloned_config = config.clone();
        assert_eq!(config, cloned_config);

        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());

        let integrator = KinematicsIntegrator::Euler;
        let cloned_integrator = integrator.clone();
        assert_eq!(integrator, cloned_integrator);

        let debug_int = format!("{:?}", integrator);
        assert!(!debug_int.is_empty());
    }
}
