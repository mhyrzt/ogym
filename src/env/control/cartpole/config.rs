use std::f64::consts::PI;

#[derive(Debug)]
pub enum KinematicsIntegrator {
    Euler,
    SemiImplicitEuler
}


#[derive(Debug)]
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
    pub integrator: KinematicsIntegrator,
}

impl CartPoleConfig {
    pub fn default() -> Self {
        CartPoleConfig {
            g: 9.8, 
            f: 10.0,
            l: 0.5,
            mc: 1.0,
            mp: 0.1,
            tau: 0.02,
            x_max: 2.4,
            t_max: 500,
            theta_max: 12.0 * 2.0 * PI / 360.0,
            integrator: KinematicsIntegrator::Euler
        }
    }
}
