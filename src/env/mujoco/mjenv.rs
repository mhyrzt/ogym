use mujoco_rs_sys::no_render;
use mujoco_rust::{Model, Simulation, State, model::ObjType};
use nalgebra as na;

use crate::env::environment::Error;

pub struct MjEnv {
    frame_skip: u32,
    state: State,
    simulation: Simulation,
    init_qpos: Vec<f64>,
    init_qvel: Vec<f64>,
}

impl MjEnv {
    pub fn new(path: &str, frame_skip: u32) -> Result<Self, Error> {
        let model = Model::from_xml(path).map_err(Error::MjInitError)?;
        let state = State::new(&model);
        let simulation = Simulation::new(model);

        let init_qpos = {
            let ptr = state.ptr();
            let nq = unsafe { (*simulation.model.ptr()).nq as usize };
            unsafe { std::slice::from_raw_parts((*ptr).qpos, nq).to_vec() }
        };

        let init_qvel = {
            let ptr = state.ptr();
            let nv = unsafe { (*simulation.model.ptr()).nv as usize };
            unsafe { std::slice::from_raw_parts((*ptr).qvel, nv).to_vec() }
        };

        Ok(Self {
            state,
            simulation,
            frame_skip,
            init_qpos,
            init_qvel,
        })
    }

    pub fn model(&self) -> &Model {
        &self.simulation.model
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
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

    pub fn dt(&self) -> f64 {
        self.timestep() * self.frame_skip as f64
    }

    pub fn time(&self) -> f64 {
        let ptr = self.state.ptr();
        unsafe { (*ptr).time }
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

    // === Dimensions ===

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

    pub fn nbody(&self) -> usize {
        self.model().nbody()
    }

    pub fn ngeom(&self) -> usize {
        self.model().ngeom()
    }

    pub fn nsite(&self) -> usize {
        unsafe { (*self.model().ptr()).nsite as usize }
    }

    pub fn ntendon(&self) -> usize {
        unsafe { (*self.model().ptr()).ntendon as usize }
    }

    pub fn init_qpos(&self) -> &[f64] {
        &self.init_qpos
    }

    pub fn init_qvel(&self) -> &[f64] {
        &self.init_qvel
    }

    pub fn qpos(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qpos, self.nq()) }
    }

    pub fn qvel(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qvel, self.nv()) }
    }

    pub fn ctrl(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).ctrl, self.nu()) }
    }

    pub fn act(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).act, self.na()) }
    }

    pub fn cfrc_ext(&self) -> &[[f64; 6]] {
        let ptr = self.state.ptr();
        let nbody = self.nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cfrc_ext as *const [f64; 6], nbody) }
    }

    pub fn cvel(&self) -> &[[f64; 6]] {
        let ptr = self.state.ptr();
        let nbody = self.nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cvel as *const [f64; 6], nbody) }
    }

    pub fn cinert(&self) -> &[[f64; 10]] {
        let ptr = self.state.ptr();
        let nbody = self.nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cinert as *const [f64; 10], nbody) }
    }

    pub fn qfrc_actuator(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qfrc_actuator, self.nv()) }
    }

    pub fn qfrc_constraint(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).qfrc_constraint, self.nv()) }
    }

    pub fn ten_velocity(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).ten_velocity, self.ntendon()) }
    }

    pub fn ten_length(&self) -> &[f64] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).ten_length, self.ntendon()) }
    }

    pub fn xipos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).xipos as *const [f64; 3], self.nbody()) }
    }

    pub fn xpos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).xpos as *const [f64; 3], self.nbody()) }
    }

    pub fn geom_xpos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).geom_xpos as *const [f64; 3], self.ngeom()) }
    }

    pub fn site_xpos(&self) -> &[[f64; 3]] {
        let ptr = self.state.ptr();
        unsafe { std::slice::from_raw_parts((*ptr).site_xpos as *const [f64; 3], self.nsite()) }
    }

    pub fn body(&self, name: &str) -> Option<[f64; 3]> {
        let id = self.model().name_to_id(ObjType::BODY, name)?;
        let ptr = self.state.ptr();
        let pos_ptr = unsafe { (*ptr).xipos.add(id as usize) };
        Some(unsafe { [*pos_ptr, *pos_ptr.add(1), *pos_ptr.add(2)] })
    }

    pub fn qpos_mut(&mut self) -> &mut [f64] {
        let ptr = self.state.ptr();
        let nq = self.nq();
        unsafe { std::slice::from_raw_parts_mut((*ptr).qpos, nq) }
    }

    pub fn qvel_mut(&mut self) -> &mut [f64] {
        let ptr = self.state.ptr();
        let nv = self.nv();
        unsafe { std::slice::from_raw_parts_mut((*ptr).qvel, nv) }
    }

    pub fn ctrl_mut(&mut self) -> &mut [f64] {
        let ptr = self.state.ptr();
        let nu = self.nu();
        unsafe { std::slice::from_raw_parts_mut((*ptr).ctrl, nu) }
    }

    pub fn act_mut(&mut self) -> &mut [f64] {
        let ptr = self.state.ptr();
        let na = self.na();
        unsafe { std::slice::from_raw_parts_mut((*ptr).act, na) }
    }

    pub fn set_state(&mut self, qpos: &[f64], qvel: &[f64]) -> Result<(), Error> {
        if qpos.len() != self.nq() {
            return Err(Error::InvalidStateDimension {
                field: "qpos",
                expected: self.nq(),
                got: qpos.len(),
            });
        }

        if qvel.len() != self.nv() {
            return Err(Error::InvalidStateDimension {
                field: "qvel",
                expected: self.nv(),
                got: qvel.len(),
            });
        }

        self.qpos_mut().copy_from_slice(qpos);
        self.qvel_mut().copy_from_slice(qvel);

        if self.na() == 0 {
            // TODO: double check (act is not used, nothing to clear)
        }

        unsafe {
            no_render::mj_forward(self.model().ptr(), self.data().ptr());
        }

        Ok(())
    }

    pub fn do_simulation(&mut self, ctrl: &[f64]) -> Result<(), Error> {
        if ctrl.len() != self.nu() {
            return Err(Error::InvalidActionDimension {
                expected: self.nu(),
                got: ctrl.len(),
            });
        }

        self.ctrl_mut().copy_from_slice(ctrl);

        for _ in 0..self.frame_skip {
            self.simulation.step();
        }

        unsafe {
            no_render::mj_rnePostConstraint(self.model().ptr(), self.data().ptr());
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.simulation.reset();
    }

    pub fn reset_to_initial(&mut self) -> Result<(), Error> {
        self.set_state(&self.init_qpos.clone(), &self.init_qvel.clone())
    }

    pub fn state_vector(&self) -> Vec<f64> {
        let mut state = Vec::with_capacity(self.nq() + self.nv());
        state.extend_from_slice(self.qpos());
        state.extend_from_slice(self.qvel());
        state
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

    pub fn act_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(self.act())
    }

    pub fn state_vector_dyn(&self) -> na::DVector<f64> {
        let nq = self.nq();
        let nv = self.nv();
        let mut state = na::DVector::zeros(nq + nv);
        state.rows_mut(0, nq).copy_from_slice(self.qpos());
        state.rows_mut(nq, nv).copy_from_slice(self.qvel());
        state
    }

    pub fn body_vector(&self, name: &str) -> Option<na::Vector3<f64>> {
        self.body(name).map(na::Vector3::from)
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

    pub fn xipos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.xipos();
        na::DMatrix::from_row_slice(data.len(), 3, unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const f64, data.len() * 3)
        })
    }

    pub fn xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.xpos();
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
}
