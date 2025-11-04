use mujoco_rust::{Model, Simulation, State, model::ObjType};
use nalgebra as na;

use crate::env::environment::Error;

pub struct MjEnv {
    state: State,
    simulation: Simulation,
}

impl MjEnv {
    pub fn new(path: &str) -> Result<Self, Error> {
        let model = Model::from_xml(path).map_err(Error::MjInitError)?;
        let state = State::new(&model);
        let simulation = Simulation::new(model);
        Ok(Self { state, simulation })
    }

    pub fn model(&self) -> &Model {
        &self.simulation.model
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn data(&self) -> &State {
        &self.state
    }

    pub fn simulation(&self) -> &Simulation {
        &self.simulation
    }

    pub fn timestep(&self) -> f64 {
        let ptr = self.model().ptr();
        unsafe { (*ptr).opt.timestep }
    }

    pub fn extent(&self) -> f64 {
        let ptr = self.model().ptr();
        unsafe { (*ptr).stat.extent }
    }

    pub fn body_mass(&self) -> &[f64] {
        let ptr = self.model().ptr();
        let n = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).body_mass, n) }
    }

    pub fn actuator_ctrlrange(&self) -> &[[f64; 2]] {
        let ptr = self.model().ptr();
        let nu = self.model().nu();
        unsafe { std::slice::from_raw_parts((*ptr).actuator_ctrlrange as *const [f64; 2], nu) }
    }

    pub fn na(&self) -> usize {
        self.model().na()
    }

    pub fn nq(&self) -> usize {
        self.model().nq()
    }

    pub fn nu(&self) -> usize {
        self.model().nu()
    }

    pub fn nv(&self) -> usize {
        self.model().nv()
    }

    pub fn qpos(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qpos, self.model().nq()) }
    }

    pub fn qvel(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qvel, self.model().nv()) }
    }

    pub fn cfrc_ext(&self) -> &[[f64; 6]] {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cfrc_ext as *const [f64; 6], nbody) }
    }

    pub fn cvel(&self) -> &[[f64; 6]] {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cvel as *const [f64; 6], nbody) }
    }

    pub fn cinert(&self) -> &[[f64; 10]] {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cinert as *const [f64; 10], nbody) }
    }

    pub fn ten_velocity(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let ntendon = unsafe { (*self.model().ptr()).ntendon as usize };
        unsafe { std::slice::from_raw_parts((*ptr).ten_velocity, ntendon) }
    }

    pub fn ten_length(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let ntendon = unsafe { (*self.model().ptr()).ntendon as usize };
        unsafe { std::slice::from_raw_parts((*ptr).ten_length, ntendon) }
    }

    pub fn ctrl(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let nu = self.model().nu();
        unsafe { std::slice::from_raw_parts((*ptr).ctrl, nu) }
    }

    pub fn qfrc_actuator(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let nv = self.model().nv();
        unsafe { std::slice::from_raw_parts((*ptr).qfrc_actuator, nv) }
    }

    pub fn qfrc_constraint(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let nv = self.model().nv();
        unsafe { std::slice::from_raw_parts((*ptr).qfrc_constraint, nv) }
    }

    pub fn act(&self) -> &[f64] {
        let ptr = self.state.ptr();
        let na = self.model().na();
        unsafe { std::slice::from_raw_parts((*ptr).act, na) }
    }

    pub fn xipos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).xipos as *const [f64; 3], nbody) }
    }

    pub fn geom_xpos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        let ngeom = self.model().ngeom();
        unsafe { std::slice::from_raw_parts((*ptr).geom_xpos as *const [f64; 3], ngeom) }
    }

    pub fn site_xpos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        let nsite = unsafe { (*self.model().ptr()).nsite as usize };
        unsafe { std::slice::from_raw_parts((*ptr).site_xpos as *const [f64; 3], nsite) }
    }

    pub fn time(&self) -> f64 {
        let ptr = self.state.ptr();
        unsafe { (*ptr).time }
    }

    pub fn body(&self, name: &str) -> Option<[f64; 3]> {
        let id = self.model().name_to_id(ObjType::BODY, name)?;
        let ptr = self.state.ptr();
        let pos_ptr = unsafe { (*ptr).xipos.add(id as usize) };
        Some(unsafe { [*pos_ptr, *pos_ptr.add(1), *pos_ptr.add(2)] })
    }

    pub fn qpos_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.qpos())
    }

    pub fn qvel_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.qvel())
    }

    pub fn ctrl_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.ctrl())
    }

    pub fn body_vector(&self, name: &str) -> Option<na::Vector3<f64>> {
        self.body(name).map(na::Vector3::from)
    }

    pub fn xipos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.xipos();
        na::DMatrix::from_row_slice(data.len(), 3, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 3)
        })
    }

    pub fn geom_xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.geom_xpos();
        na::DMatrix::from_row_slice(data.len(), 3, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 3)
        })
    }

    pub fn site_xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.site_xpos();
        na::DMatrix::from_row_slice(data.len(), 3, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 3)
        })
    }

    pub fn cvel_matrix(&self) -> na::DMatrix<f64> {
        let data = self.cvel();
        na::DMatrix::from_row_slice(data.len(), 6, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 6)
        })
    }

    pub fn cfrc_ext_matrix(&self) -> na::DMatrix<f64> {
        let data = self.cfrc_ext();
        na::DMatrix::from_row_slice(data.len(), 6, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 6)
        })
    }

    pub fn qfrc_actuator_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.qfrc_actuator())
    }

    pub fn qfrc_constraint_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.qfrc_constraint())
    }

    pub fn ten_velocity_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.ten_velocity())
    }

    pub fn ten_length_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.ten_length())
    }

    pub fn act_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.act())
    }
}
