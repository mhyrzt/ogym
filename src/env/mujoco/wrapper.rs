use mujoco_rust::{Model, Simulation, State};

pub struct MujocoEnvironment {
    state: State,
    simulation: Simulation,
}

impl MujocoEnvironment {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let model = Model::from_xml(path).map_err(|err| anyhow::anyhow!(err))?;
        let state = State::new(&model);
        let simulation = Simulation::new(model);
        Ok(Self { state, simulation })
    }

    // ---------- model ------------
    pub fn model(&self) -> &Model {
        &self.simulation.model
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

    // --------- DATA or state ---------------

    /// Generalized positions
    pub fn qpos(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).qpos, self.model().nq()) })
    }

    /// Generalized velocities
    pub fn qvel(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).qvel, self.model().nv()) })
    }

    /// Constraint forces
    pub fn cfrc_ext(&self) -> Vec<[f64; 6]> {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cfrc_ext as *const [f64; 6], nbody).to_vec() }
    }

    /// Cartesion velocities of all bodies
    pub fn cvel(&self) -> Vec<[f64; 6]> {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cvel as *const [f64; 6], nbody).to_vec() }
    }

    /// Composite body inertia and mass (mjData.cinert; shape [nbody, 10])
    pub fn cinert(&self) -> Vec<[f64; 10]> {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).cinert as *const [f64; 10], nbody).to_vec() }
    }

    /// Tendon velocities (mjData.ten_velocity; size = ntendon)
    pub fn ten_velocity(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let ntendon = unsafe { (*self.model().ptr()).ntendon as usize };
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).ten_velocity, ntendon) })
    }

    /// Tendon lengths (mjData.ten_length; size = ntendon)
    pub fn ten_length(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let ntendon = unsafe { (*self.model().ptr()).ntendon as usize };
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).ten_length, ntendon) })
    }

    /// Control inputs (mjData.ctrl; size = nu)
    pub fn ctrl(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let nu = self.model().nu();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).ctrl, nu) })
    }

    /// Actuator forces (mjData.qfrc_actuator; size = nv)
    pub fn qfrc_actuator(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let nv = self.model().nv();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).qfrc_actuator, nv) })
    }

    /// External constraint forces (mjData.qfrc_constraint; size = nv)
    pub fn qfrc_constraint(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let nv = self.model().nv();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).qfrc_constraint, nv) })
    }

    /// Copy of `mjData.act` (actuator activation; size = na)
    pub fn act(&self) -> Vec<f64> {
        let ptr = self.state.ptr();
        let na = self.model().na();
        Vec::from(unsafe { std::slice::from_raw_parts((*ptr).act, na) })
    }

    /// Body positions in Cartesian (global) coordinates (mjData.xipos; [nbody, 3])
    pub fn xipos(&self) -> Vec<[f64; 3]> {
        let ptr = self.state.ptr();
        let nbody = self.model().nbody();
        unsafe { std::slice::from_raw_parts((*ptr).xipos as *const [f64; 3], nbody).to_vec() }
    }

    /// Geom positions in Cartesian (global) coordinates (mjData.geom_xpos; [ngeom, 3])
    pub fn geom_xpos(&self) -> Vec<[f64; 3]> {
        let ptr = self.state.ptr();
        let ngeom = self.model().ngeom();
        unsafe { std::slice::from_raw_parts((*ptr).geom_xpos as *const [f64; 3], ngeom).to_vec() }
    }

    /// Site positions in Cartesian (global) coordinates (mjData.site_xpos; [nsite, 3])
    pub fn site_xpos(&self) -> Vec<[f64; 3]> {
        let ptr = self.state.ptr();
        let nsite = unsafe { (*self.model().ptr()).nsite as usize };
        unsafe { std::slice::from_raw_parts((*ptr).site_xpos as *const [f64; 3], nsite).to_vec() }
    }

    /// Simulation time
    pub fn time(&self) -> f64 {
        let ptr = self.state.ptr();
        unsafe { (*ptr).time }
    }

    pub fn body(&self) {
        todo!()
    }
}
