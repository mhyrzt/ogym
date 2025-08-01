#[derive(Debug, Copy, Clone)]
pub struct LunarLanderConfig {
    pub scale: f64,
    pub fps: i32,
    pub max_steps: u32,
    pub main_engine_force: f64,
    pub side_engine_force: f64,
    pub side_engine_offset_x: f64,
    pub side_engine_offset_y: f64,
    pub leg_offset_x: f64,
    pub leg_offset_y: f64,
    pub leg_width: f64,
    pub leg_length: f64,
    pub leg_spring_torque: f64,
    pub main_engine_y_position: f64,
    pub gravity: f64,
    pub continuous: bool,
    pub wind_strength: Option<f64>,
    pub turbulence_strength: f64,
    pub initial_random: f64,
    pub viewport_width: i32,
    pub viewport_height: i32,
}

impl Default for LunarLanderConfig {
    fn default() -> Self {
        Self {
            scale: 30.0,
            fps: 50,
            max_steps: 200,
            main_engine_force: 13.0,
            side_engine_force: 0.6,
            side_engine_offset_x: 12.0,
            side_engine_offset_y: 14.0, // Fixed: was 8.0, should be 14.0
            leg_offset_x: 20.0,
            leg_offset_y: 18.0,
            leg_width: 2.0,
            leg_length: 8.0, // Fixed: was 9.0, should be 8.0
            leg_spring_torque: 40.0,
            main_engine_y_position: 4.0,
            gravity: -10.0,
            continuous: false,
            wind_strength: None, // Default disabled, can set to Some(15.0) to match Python default
            turbulence_strength: 1.5,
            initial_random: 1000.0,
            viewport_width: 600,
            viewport_height: 400,
        }
    }
}

impl LunarLanderConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_main_engine_force(mut self, force: f64) -> Self {
        self.main_engine_force = force;
        self
    }

    pub fn with_side_engine_force(mut self, force: f64) -> Self {
        self.side_engine_force = force;
        self
    }

    pub fn with_side_engine_offset(mut self, x: f64, y: f64) -> Self {
        self.side_engine_offset_x = x;
        self.side_engine_offset_y = y;
        self
    }

    pub fn with_leg_offset(mut self, x: f64, y: f64) -> Self {
        self.leg_offset_x = x;
        self.leg_offset_y = y;
        self
    }

    pub fn with_leg_size(mut self, width: f64, length: f64) -> Self {
        self.leg_width = width;
        self.leg_length = length;
        self
    }

    pub fn with_main_engine_y_position(mut self, pos: f64) -> Self {
        self.main_engine_y_position = pos;
        self
    }

    pub fn with_gravity(mut self, g: f64) -> Self {
        self.gravity = g;
        self
    }

    pub fn with_discrete_action(mut self) -> Self {
        self.continuous = false;
        self
    }

    pub fn with_continuous_action(mut self) -> Self {
        self.continuous = true;
        self
    }

    pub fn with_wind_strength(mut self, wind: f64) -> Self {
        self.wind_strength = Some(wind);
        self
    }

    pub fn without_wind(mut self) -> Self {
        self.wind_strength = None;
        self
    }

    pub fn with_turbulence_strength(mut self, turbulence: f64) -> Self {
        self.turbulence_strength = turbulence;
        self
    }

    pub fn with_fps(mut self, fps: i32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_leg_spring_torque(mut self, torque: f64) -> Self {
        self.leg_spring_torque = torque;
        self
    }

    pub fn with_viewport_size(mut self, width: i32, height: i32) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self
    }

    pub fn get_scaled_width(&self) -> f64 {
        self.viewport_width as f64 / self.scale
    }

    pub fn get_scaled_height(&self) -> f64 {
        self.viewport_height as f64 / self.scale
    }
}
