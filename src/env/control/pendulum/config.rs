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