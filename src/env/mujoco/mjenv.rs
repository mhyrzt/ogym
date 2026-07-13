use mujoco_rs_sys::no_render;
use mujoco_rust::{model::ObjType, Model, Simulation, State};
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
    pub fn new(xml: impl AsRef<str>, frame_skip: u32) -> Result<Self, Error> {
        let model = Model::from_xml_str(xml).map_err(Error::MjInitError)?;
        let simulation = Simulation::new(model);
        let state = State::new(&simulation.model);

        unsafe {
            no_render::mj_resetData(simulation.model.ptr(), state.ptr());
            no_render::mj_forward(simulation.model.ptr(), state.ptr());
        }

        let init_qpos = {
            let ptr = state.ptr();
            let nq = simulation.model.nq();
            unsafe { std::slice::from_raw_parts((*ptr).qpos, nq).to_vec() }
        };

        let init_qvel = {
            let ptr = state.ptr();
            let nv = simulation.model.nv();
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

        if (id as usize) >= self.nbody() {
            return None;
        }

        let ptr = self.state.ptr();

        unsafe {
            let pos_ptr = (*ptr).xpos.add((id as usize) * 3);
            Some([*pos_ptr, *pos_ptr.add(1), *pos_ptr.add(2)])
        }
    }

    /// Center-of-mass position (MuJoCo's `xipos`) of the body with the given
    /// name, resolved by name rather than a hard-coded body index. This is
    /// what Gymnasium's `get_body_com(name)` reads, as opposed to `body()`
    /// above (`xpos`, the body frame origin) which can differ from the COM
    /// whenever a body's geoms aren't centered on its frame origin.
    pub fn body_com(&self, name: &str) -> Option<[f64; 3]> {
        let id = self.model().name_to_id(ObjType::BODY, name)?;

        if (id as usize) >= self.nbody() {
            return None;
        }

        let ptr = self.state.ptr();

        unsafe {
            let pos_ptr = (*ptr).xipos.add((id as usize) * 3);
            Some([*pos_ptr, *pos_ptr.add(1), *pos_ptr.add(2)])
        }
    }

    pub fn body_com_vector(&self, name: &str) -> Option<na::Vector3<f64>> {
        self.body_com(name).map(na::Vector3::from)
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

        let model_ptr = self.model().ptr();
        let data_ptr = self.state.ptr();

        unsafe {
            let frame_skip = self.frame_skip;
            for _ in 0..frame_skip {
                no_render::mj_step(model_ptr, data_ptr);
            }
            no_render::mj_rnePostConstraint(model_ptr, data_ptr);
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        unsafe {
            no_render::mj_resetData(self.model().ptr(), self.state.ptr());
            no_render::mj_forward(self.model().ptr(), self.state.ptr());
        }
    }

    pub fn reset_to_initial(&mut self) -> Result<(), Error> {
        let qpos = self.init_qpos.clone();
        let qvel = self.init_qvel.clone();
        self.set_state(&qpos, &qvel)
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

    fn slice_arrays_to_matrix(rows: usize, cols: usize, data: &[f64]) -> na::DMatrix<f64> {
        na::DMatrix::from_row_slice(rows, cols, data)
    }

    pub fn xipos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.xipos();
        let flat_len = data.len() * 3;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 3, flat_data)
    }

    pub fn xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.xpos();
        let flat_len = data.len() * 3;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 3, flat_data)
    }

    pub fn geom_xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.geom_xpos();
        let flat_len = data.len() * 3;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 3, flat_data)
    }

    pub fn site_xpos_matrix(&self) -> na::DMatrix<f64> {
        let data = self.site_xpos();
        let flat_len = data.len() * 3;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 3, flat_data)
    }

    pub fn cvel_matrix(&self) -> na::DMatrix<f64> {
        let data = self.cvel();
        let flat_len = data.len() * 6;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 6, flat_data)
    }

    pub fn cfrc_ext_matrix(&self) -> na::DMatrix<f64> {
        let data = self.cfrc_ext();
        let flat_len = data.len() * 6;
        let flat_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f64, flat_len) };
        Self::slice_arrays_to_matrix(data.len(), 6, flat_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MODEL_XML: &str = r#"
    <mujoco model="simple_test">
        <compiler angle="radian"/>
        <option timestep="0.01" gravity="0 0 -9.81"/>
        <worldbody>
            <light pos="0 0 3" dir="0 0 -1" />
            <geom type="plane" size="10 10 0.1" rgba=".9 .9 .9 1"/>
            <!-- Slider body starting at Z=1 -->
            <body name="slider_body" pos="0 0 1">
                <joint name="slide_x" type="slide" axis="1 0 0" />
                <geom type="box" size="0.1 0.1 0.1" mass="1.0" />
            </body>
        </worldbody>
        <actuator>
            <motor name="motor_x" joint="slide_x" gear="10.0" />
        </actuator>
    </mujoco>
    "#;

    const INVALID_XML: &str = "Not an XML string";

    #[test]
    fn test_initialization_success() {
        let frame_skip = 4;
        let env = MjEnv::new(TEST_MODEL_XML, frame_skip);

        assert!(
            env.is_ok(),
            "MjEnv should initialize successfully with valid XML content"
        );

        let env = env.unwrap();

        assert_eq!(env.nq(), 1, "Expected 1 generalized coordinate (qpos)");
        assert_eq!(env.nv(), 1, "Expected 1 generalized velocity (qvel)");
        assert_eq!(env.nu(), 1, "Expected 1 control input (nu)");

        assert_eq!(env.nbody(), 2, "Expected 2 bodies (world + slider)");

        let expected_dt = 0.01 * frame_skip as f64;
        assert!(
            (env.dt() - expected_dt).abs() < 1e-6,
            "DT calculation incorrect"
        );
    }

    #[test]
    fn test_initialization_failure() {
        let env = MjEnv::new(INVALID_XML, 1);
        assert!(
            env.is_err(),
            "MjEnv should fail when initialized with invalid XML"
        );

        match env {
            Err(Error::MjInitError(_)) => (),
            _ => panic!("Expected MjInitError"),
        }
    }

    #[test]
    fn test_body_lookup_and_position() {
        let env = MjEnv::new(TEST_MODEL_XML, 1).unwrap();

        let pos = env.body_vector("slider_body");
        assert!(pos.is_some(), "Should find 'slider_body'");

        let pos_vec = pos.unwrap();
        assert!((pos_vec.x - 0.0).abs() < 1e-6);
        assert!((pos_vec.y - 0.0).abs() < 1e-6);
        assert!((pos_vec.z - 1.0).abs() < 1e-6);

        // Lookup non-existent body
        let missing = env.body_vector("non_existent_ghost");
        assert!(
            missing.is_none(),
            "Should return None for invalid body name"
        );
    }

    #[test]
    fn test_set_state_and_reset() {
        let mut env = MjEnv::new(TEST_MODEL_XML, 1).unwrap();

        let new_qpos = vec![0.5];
        let new_qvel = vec![0.1];

        let res = env.set_state(&new_qpos, &new_qvel);
        assert!(res.is_ok());

        assert!((env.qpos()[0] - 0.5).abs() < 1e-6, "qpos not updated");
        assert!((env.qvel()[0] - 0.1).abs() < 1e-6, "qvel not updated");

        let bad_qpos = vec![0.5, 0.5];
        let res_bad = env.set_state(&bad_qpos, &new_qvel);
        assert!(res_bad.is_err(), "Should fail with wrong qpos dimensions");

        let reset_res = env.reset_to_initial();
        assert!(reset_res.is_ok());

        assert!((env.qpos()[0] - 0.0).abs() < 1e-6, "qpos not reset to 0");
        assert!((env.qvel()[0] - 0.0).abs() < 1e-6, "qvel not reset to 0");
    }

    #[test]
    fn test_simulation_integration() {
        let frame_skip = 1;
        let mut env = MjEnv::new(TEST_MODEL_XML, frame_skip).unwrap();

        let start_time = env.time();

        let action = vec![1.0];

        let step_res = env.do_simulation(&action);
        assert!(step_res.is_ok());

        let end_time = env.time();
        assert!(
            end_time > start_time,
            "Time should advance after simulation step"
        );
        assert!(
            (end_time - start_time - 0.01).abs() < 1e-6,
            "Time advanced by incorrect amount"
        );

        let qvel_after = env.qvel()[0];
        assert!(
            qvel_after > 0.0,
            "Applying positive motor force should result in positive velocity"
        );

        // Check that control was stored
        assert_eq!(env.ctrl()[0], 1.0);
    }

    #[test]
    fn test_matrix_data_access() {
        let env = MjEnv::new(TEST_MODEL_XML, 1).unwrap();

        let xpos_mat = env.xpos_matrix();
        let (rows, cols) = xpos_mat.shape();

        assert_eq!(cols, 3, "xpos matrix should have 3 columns (x,y,z)");
        assert_eq!(rows, env.nbody(), "xpos matrix rows should match nbody");

        let geom_mat = env.geom_xpos_matrix();
        assert_eq!(geom_mat.shape().0, env.ngeom());
        assert_eq!(geom_mat.shape().1, 3);

        let cvel_mat = env.cvel_matrix();
        assert_eq!(cvel_mat.shape().0, env.nbody());
        assert_eq!(cvel_mat.shape().1, 6);
    }

    #[test]
    fn test_state_vector_concatenation() {
        let mut env = MjEnv::new(TEST_MODEL_XML, 1).unwrap();

        let qpos = vec![2.0];
        let qvel = vec![1.0];
        let _ = env.set_state(&qpos, &qvel);

        let vec_dyn = env.state_vector_dyn();

        assert_eq!(vec_dyn.len(), 2);
        assert!((vec_dyn[0] - 2.0).abs() < 1e-6);
        assert!((vec_dyn[1] - 1.0).abs() < 1e-6);
    }
}
