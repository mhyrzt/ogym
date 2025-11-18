use super::config::Walker2dConfig;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};

pub struct MujocoWalker2dEnv {
    pub env: MjEnv,
    pub config: Walker2dConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
}

impl MujocoWalker2dEnv {
    pub fn new(config: Option<Walker2dConfig>) -> Result<Self, Error> {
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
