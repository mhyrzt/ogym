use std::f64::consts::PI;

use nalgebra::SVector;

use crate::{env::{Environment, Error}, spaces::{Boxed, EnvSpace, Mixed, MixedItem}};

use super::{config::DynamicsMode, AcrobotConfig};

const ACTION_SIZE: usize = 1; // Discrete(3) or optionally continuous
const RAW_STATE_SIZE: usize = 4;
const STATE_SIZE: usize = 6;

type RawState = SVector<f64, RAW_STATE_SIZE>;
type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;



#[derive(Debug)]
pub struct Acrobot {
    pub config: AcrobotConfig,
    t: u32,
    raw_state: Option<RawState>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

