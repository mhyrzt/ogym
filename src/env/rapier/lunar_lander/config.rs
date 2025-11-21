#[derive(Debug, Copy, Clone)]
pub struct LunarLanderConfig {
    pub scale: f32,
    pub fps: i32,
    pub max_steps: u32,
    pub main_engine_force: f32,
    pub side_engine_force: f32,
    pub side_engine_offset_x: f32,
    pub side_engine_offset_y: f32,
    pub leg_offset_x: f32,
    pub leg_offset_y: f32,
    pub leg_width: f32,
    pub leg_length: f32,
    pub leg_spring_torque: f32,
    pub main_engine_y_position: f32,
    pub gravity: f32,
    pub continuous: bool,
    pub wind_strength: Option<f32>,
    pub turbulence_strength: f32,
    pub initial_random: f32,
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
            side_engine_offset_y: 14.0,
            leg_offset_x: 20.0,
            leg_offset_y: 18.0,
            leg_width: 2.0,
            leg_length: 8.0,
            leg_spring_torque: 40.0,
            main_engine_y_position: 4.0,
            gravity: -10.0,
            continuous: false,
            wind_strength: None,
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

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_main_engine_force(mut self, force: f32) -> Self {
        self.main_engine_force = force;
        self
    }

    pub fn with_side_engine_force(mut self, force: f32) -> Self {
        self.side_engine_force = force;
        self
    }

    pub fn with_side_engine_offset(mut self, x: f32, y: f32) -> Self {
        self.side_engine_offset_x = x;
        self.side_engine_offset_y = y;
        self
    }

    pub fn with_leg_offset(mut self, x: f32, y: f32) -> Self {
        self.leg_offset_x = x;
        self.leg_offset_y = y;
        self
    }

    pub fn with_leg_size(mut self, width: f32, length: f32) -> Self {
        self.leg_width = width;
        self.leg_length = length;
        self
    }

    pub fn with_main_engine_y_position(mut self, pos: f32) -> Self {
        self.main_engine_y_position = pos;
        self
    }

    pub fn with_gravity(mut self, g: f32) -> Self {
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

    pub fn with_wind_strength(mut self, wind: f32) -> Self {
        self.wind_strength = Some(wind);
        self
    }

    pub fn without_wind(mut self) -> Self {
        self.wind_strength = None;
        self
    }

    pub fn with_initial_random(mut self, init_rand: f32) -> Self {
        self.initial_random = init_rand;
        self
    }

    pub fn with_turbulence_strength(mut self, turbulence: f32) -> Self {
        self.turbulence_strength = turbulence;
        self
    }

    pub fn with_fps(mut self, fps: i32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_leg_spring_torque(mut self, torque: f32) -> Self {
        self.leg_spring_torque = torque;
        self
    }

    pub fn with_viewport_size(mut self, width: i32, height: i32) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self
    }

    pub fn get_scaled_width(&self) -> f32 {
        self.viewport_width as f32 / self.scale
    }

    pub fn get_scaled_height(&self) -> f32 {
        self.viewport_height as f32 / self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
        let config = LunarLanderConfig::default();

        assert_eq!(config.scale, 30.0);
        assert_eq!(config.fps, 50);
        assert_eq!(config.max_steps, 200);
        assert_eq!(config.main_engine_force, 13.0);
        assert_eq!(config.side_engine_force, 0.6);
        assert_eq!(config.side_engine_offset_x, 12.0);
        assert_eq!(config.side_engine_offset_y, 14.0);
        assert_eq!(config.leg_offset_x, 20.0);
        assert_eq!(config.leg_offset_y, 18.0);
        assert_eq!(config.leg_width, 2.0);
        assert_eq!(config.leg_length, 8.0);
        assert_eq!(config.leg_spring_torque, 40.0);
        assert_eq!(config.main_engine_y_position, 4.0);
        assert_eq!(config.gravity, -10.0);
        assert!(!config.continuous);
        assert_eq!(config.wind_strength, None);
        assert_eq!(config.turbulence_strength, 1.5);
        assert_eq!(config.initial_random, 1000.0);
        assert_eq!(config.viewport_width, 600);
        assert_eq!(config.viewport_height, 400);
    }

    #[test]
    fn test_new_alias() {
        let config_default = LunarLanderConfig::default();
        let config_new = LunarLanderConfig::new();
        // Verify new() produces identical output to default() by checking a few key fields
        assert_eq!(config_default.scale, config_new.scale);
        assert_eq!(config_default.fps, config_new.fps);
    }

    #[test]
    fn test_builder_simple_setters() {
        let config = LunarLanderConfig::new()
            .with_scale(45.0)
            .with_main_engine_force(20.0)
            .with_side_engine_force(1.5)
            .with_main_engine_y_position(5.5)
            .with_gravity(-9.81)
            .with_turbulence_strength(0.5)
            .with_fps(60)
            .with_leg_spring_torque(50.0);

        assert_eq!(config.scale, 45.0);
        assert_eq!(config.main_engine_force, 20.0);
        assert_eq!(config.side_engine_force, 1.5);
        assert_eq!(config.main_engine_y_position, 5.5);
        assert_eq!(config.gravity, -9.81);
        assert_eq!(config.turbulence_strength, 0.5);
        assert_eq!(config.fps, 60);
        assert_eq!(config.leg_spring_torque, 50.0);
    }

    #[test]
    fn test_builder_compound_setters() {
        let config = LunarLanderConfig::new()
            .with_side_engine_offset(10.0, 15.0)
            .with_leg_offset(25.0, 20.0)
            .with_leg_size(3.0, 10.0)
            .with_viewport_size(800, 600);

        assert_eq!(config.side_engine_offset_x, 10.0);
        assert_eq!(config.side_engine_offset_y, 15.0);

        assert_eq!(config.leg_offset_x, 25.0);
        assert_eq!(config.leg_offset_y, 20.0);

        assert_eq!(config.leg_width, 3.0);
        assert_eq!(config.leg_length, 10.0);

        assert_eq!(config.viewport_width, 800);
        assert_eq!(config.viewport_height, 600);
    }

    #[test]
    fn test_action_space_configuration() {
        // Default is discrete
        let mut config = LunarLanderConfig::default();
        assert!(!config.continuous);

        // Switch to continuous
        config = config.with_continuous_action();
        assert!(config.continuous);

        // Switch back to discrete
        config = config.with_discrete_action();
        assert!(!config.continuous);
    }

    #[test]
    fn test_wind_configuration() {
        // Default is None
        let mut config = LunarLanderConfig::default();
        assert!(config.wind_strength.is_none());

        // Enable wind
        config = config.with_wind_strength(5.0);
        assert_eq!(config.wind_strength, Some(5.0));

        // Disable wind
        config = config.without_wind();
        assert!(config.wind_strength.is_none());
    }

    #[test]
    fn test_scaled_dimension_calculations() {
        let config = LunarLanderConfig::new()
            .with_viewport_size(600, 300)
            .with_scale(30.0);

        let expected_width = 600.0 / 30.0; // 20.0
        let expected_height = 300.0 / 30.0; // 10.0

        assert_eq!(config.get_scaled_width(), expected_width);
        assert_eq!(config.get_scaled_height(), expected_height);
    }

    #[test]
    fn test_traits() {
        let config = LunarLanderConfig::default();

        // Test Debug
        let debug_output = format!("{:?}", config);
        assert!(debug_output.contains("LunarLanderConfig"));
        assert!(debug_output.contains("scale: 30.0"));

        // Test Clone and Copy
        let config_copy = config;
        assert_eq!(config.scale, config_copy.scale);

        let config_clone = config;
        assert_eq!(config.scale, config_clone.scale);
    }
}
