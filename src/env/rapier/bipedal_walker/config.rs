use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipedalWalkerConfig {
    pub fps: u32,
    pub scale: f32,

    // Motor parameters
    pub motors_torque: f32,
    pub speed_hip: f32,
    pub speed_knee: f32,

    // Lidar parameters
    pub lidar_range: f32,
    pub lidar_count: usize,

    // Initial conditions
    pub initial_random: f32,

    pub hull_vertices: Vec<(f32, f32)>,

    pub leg_down: f32,
    pub leg_w: f32,
    pub leg_h: f32,

    pub viewport_w: f32,
    pub viewport_h: f32,

    pub terrain_step: f32,
    pub terrain_length: usize,
    pub terrain_height: f32,
    pub terrain_grass: usize,
    pub terrain_startpad: usize,
    pub friction: f32,

    pub max_episode_steps: u32,
    pub hardcore: bool,

    pub control_speed: bool,
}

impl Default for BipedalWalkerConfig {
    fn default() -> Self {
        let scale = 30.0;
        let viewport_h = 400.0;

        Self {
            fps: 50,
            scale,
            motors_torque: 80.0,
            speed_hip: 4.0,
            speed_knee: 6.0,
            lidar_range: 160.0 / scale,
            lidar_count: 10,
            initial_random: 5.0,
            hull_vertices: vec![
                (-30.0, 9.0),
                (6.0, 9.0),
                (34.0, 1.0),
                (34.0, -8.0),
                (-30.0, -8.0),
            ],
            leg_down: -8.0 / scale,
            leg_w: 8.0 / scale,
            leg_h: 34.0 / scale,
            viewport_w: 600.0,
            viewport_h,
            terrain_step: 14.0 / scale,
            terrain_length: 200,
            terrain_height: viewport_h / scale / 4.0,
            terrain_grass: 10,
            terrain_startpad: 20,
            friction: 2.5,
            max_episode_steps: 1600,
            hardcore: false,
            control_speed: false,
        }
    }
}

impl BipedalWalkerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hardcore(mut self, hardcore: bool) -> Self {
        self.hardcore = hardcore;
        self
    }

    pub fn with_control_speed(mut self, control_speed: bool) -> Self {
        self.control_speed = control_speed;
        self
    }

    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_episode_steps = max_steps;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_hull_vertices(mut self, vertices: Vec<(f32, f32)>) -> Self {
        self.hull_vertices = vertices;
        self
    }

    pub fn get_scaled_hull_vertices(&self) -> Vec<Vector2<f32>> {
        self.hull_vertices
            .iter()
            .map(|(x, y)| Vector2::new(x / self.scale, y / self.scale))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;

    #[test]
    fn test_default_configuration_values() {
        let config = BipedalWalkerConfig::default();

        assert_eq!(config.fps, 50);
        assert_eq!(config.scale, 30.0);
        assert_eq!(config.motors_torque, 80.0);
        assert_eq!(config.speed_hip, 4.0);
        assert_eq!(config.speed_knee, 6.0);

        // Calculated fields based on scale 30.0
        assert_eq!(config.lidar_range, 160.0 / 30.0);
        assert_eq!(config.lidar_count, 10);
        assert_eq!(config.initial_random, 5.0);

        assert_eq!(config.hull_vertices.len(), 5);
        assert_eq!(config.hull_vertices[0], (-30.0, 9.0));

        assert_eq!(config.leg_down, -8.0 / 30.0);
        assert_eq!(config.leg_w, 8.0 / 30.0);
        assert_eq!(config.leg_h, 34.0 / 30.0);

        assert_eq!(config.viewport_w, 600.0);
        assert_eq!(config.viewport_h, 400.0);

        assert_eq!(config.terrain_step, 14.0 / 30.0);
        assert_eq!(config.terrain_length, 200);
        assert_eq!(config.terrain_height, 400.0 / 30.0 / 4.0);
        assert_eq!(config.terrain_grass, 10);
        assert_eq!(config.terrain_startpad, 20);
        assert_eq!(config.friction, 2.5);

        assert_eq!(config.max_episode_steps, 1600);
        assert!(!config.hardcore);
        assert!(!config.control_speed);
    }

    #[test]
    fn test_new_same_as_default() {
        let config_new = BipedalWalkerConfig::new();
        let config_default = BipedalWalkerConfig::default();

        // Using debug format for comparison since partialeq isn't derived
        assert_eq!(format!("{:?}", config_new), format!("{:?}", config_default));
    }

    #[test]
    fn test_builder_methods() {
        let config = BipedalWalkerConfig::new()
            .with_hardcore(true)
            .with_control_speed(true)
            .with_fps(60)
            .with_max_steps(2000);

        assert!(config.hardcore);
        assert!(config.control_speed);
        assert_eq!(config.fps, 60);
        assert_eq!(config.max_episode_steps, 2000);
    }

    #[test]
    fn test_builder_chaining_order() {
        let config = BipedalWalkerConfig::new().with_fps(100).with_fps(30);

        assert_eq!(config.fps, 30);
    }

    #[test]
    fn test_get_scaled_hull_vertices() {
        let config = BipedalWalkerConfig::default()
            .with_scale(2.0)
            .with_hull_vertices(vec![(10.0, 20.0), (4.0, -8.0)]);

        let scaled = config.get_scaled_hull_vertices();

        assert_eq!(scaled.len(), 2);
        assert_eq!(scaled[0], Vector2::new(5.0, 10.0));
        assert_eq!(scaled[1], Vector2::new(2.0, -4.0));
    }

    #[test]
    fn test_default_scaled_hull_vertices() {
        let config = BipedalWalkerConfig::default();
        let scaled = config.get_scaled_hull_vertices();
        let scale = 30.0;

        assert_eq!(scaled.len(), 5);
        assert_eq!(scaled[0], Vector2::new(-30.0 / scale, 9.0 / scale));
        assert_eq!(scaled[1], Vector2::new(6.0 / scale, 9.0 / scale));
        assert_eq!(scaled[2], Vector2::new(34.0 / scale, 1.0 / scale));
        assert_eq!(scaled[3], Vector2::new(34.0 / scale, -8.0 / scale));
        assert_eq!(scaled[4], Vector2::new(-30.0 / scale, -8.0 / scale));
    }

    #[test]
    fn test_serialization() {
        let config = BipedalWalkerConfig::default();
        let serialized = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: BipedalWalkerConfig =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(config.fps, deserialized.fps);
        assert_eq!(config.scale, deserialized.scale);
        assert_eq!(config.hull_vertices.len(), deserialized.hull_vertices.len());
    }

    #[test]
    fn test_clone() {
        let config = BipedalWalkerConfig::new();
        let cloned = config.clone();

        assert_eq!(config.fps, cloned.fps);
        assert_eq!(config.hull_vertices.len(), cloned.hull_vertices.len());
    }
}
