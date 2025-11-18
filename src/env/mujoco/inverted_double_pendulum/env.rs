use nalgebra::DVector;
use crate::env::{environment::Error, mujoco::mjenv::MjEnv};
use super::config::InvertedDoublePendulumConfig;

pub struct MujocoInvertedDoublePendulumEnv {
    pub env: MjEnv,
    pub config: InvertedDoublePendulumConfig,
    pub init_qpos: Vec<f64>,
    pub init_qvel: Vec<f64>,
}

impl MujocoInvertedDoublePendulumEnv {
    pub fn new(config: Option<InvertedDoublePendulumConfig>) -> Result<Self, Error> {
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