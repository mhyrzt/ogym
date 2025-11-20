use super::config::PusherConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use nalgebra::DVector;

pub struct MujocoPusherEnv {
    pub env: MjEnv,
    pub config: PusherConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
}

impl MujocoPusherEnv {
    pub fn new(config: Option<PusherConfig>) -> Result<Self, Error> {
        let config = config.unwrap_or_default();
        let env = MjEnv::new(&config.xml_file, config.frame_skip)?;
        Ok(Self {
            init_qpos: env.init_qpos().into(),
            init_qvel: env.init_qvel().into(),
            config,
            env,
        })
    }
}
