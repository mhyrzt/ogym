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
    pub fn builder() -> Self {
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
