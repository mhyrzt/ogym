use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipedalWalkerConfig {
    pub fps: u32,
    pub scale: f64,

    // Motor parameters
    pub motors_torque: f64,
    pub speed_hip: f64,
    pub speed_knee: f64,

    // Lidar parameters
    pub lidar_range: f64,
    pub lidar_count: usize,

    // Initial conditions
    pub initial_random: f64,

    pub hull_vertices: Vec<(f64, f64)>,

    pub leg_down: f64,
    pub leg_w: f64,
    pub leg_h: f64,

    pub viewport_w: f64,
    pub viewport_h: f64,

    pub terrain_step: f64,
    pub terrain_length: usize,
    pub terrain_height: f64,
    pub terrain_grass: usize,
    pub terrain_startpad: usize,
    pub friction: f64,

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

    pub fn get_scaled_hull_vertices(&self) -> Vec<Vector2<f32>> {
        self.hull_vertices
            .iter()
            .map(|(x, y)| Vector2::new((x / self.scale) as f32, (y / self.scale) as f32))
            .collect()
    }
}
