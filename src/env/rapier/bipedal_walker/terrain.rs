use nalgebra::{Point2, Vector2};
use rand::Rng;
use rapier2d::prelude::*;

use crate::env::rapier::bipedal_walker::config::BipedalWalkerConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerrainState {
    Grass,
    Stump,
    Stairs,
    Pit,
}

pub struct TerrainGenerator {
    config: BipedalWalkerConfig,
}

impl TerrainGenerator {
    pub fn new(config: BipedalWalkerConfig) -> Self {
        Self { config }
    }

    pub fn generate<R: Rng>(
        &self,
        rng: &mut R,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        hardcore: bool,
    ) -> (Vec<RigidBodyHandle>, Vec<f32>, Vec<f32>) {
        let mut terrain_handles = Vec::new();
        let mut terrain_x = Vec::new();
        let mut terrain_y = Vec::new();

        let mut state = TerrainState::Grass;
        let mut velocity = 0.0;
        let mut y = self.config.terrain_height;
        let mut counter = self.config.terrain_startpad as i32;
        let mut oneshot = false;

        let mut stair_steps = 0i32;
        let mut stair_width = 0i32;
        let mut stair_height = 0i32;
        let mut original_y = 0.0;

        for i in 0..self.config.terrain_length {
            let x = i as f32 * self.config.terrain_step;
            terrain_x.push(x);

            match state {
                TerrainState::Grass if !oneshot => {
                    velocity = 0.8 * velocity + 0.01 * (self.config.terrain_height - y).signum();
                    if i > self.config.terrain_startpad {
                        velocity += rng.random_range(-1.0..1.0) / self.config.scale;
                    }
                    y += velocity;
                }

                TerrainState::Pit if oneshot => {
                    counter = rng.random_range(3i32..5i32);
                    let step = self.config.terrain_step;

                    // Create left wall
                    let poly = vec![
                        Vector2::new(x, y),
                        Vector2::new(x + step, y),
                        Vector2::new(x + step, y - 4.0 * step),
                        Vector2::new(x, y - 4.0 * step),
                    ];
                    self.create_polygon_terrain(
                        rigid_body_set,
                        collider_set,
                        &poly,
                        &mut terrain_handles,
                    );

                    // Create right wall
                    let offset = step * counter as f32;
                    let poly = vec![
                        Vector2::new(x + offset, y),
                        Vector2::new(x + offset + step, y),
                        Vector2::new(x + offset + step, y - 4.0 * step),
                        Vector2::new(x + offset, y - 4.0 * step),
                    ];
                    self.create_polygon_terrain(
                        rigid_body_set,
                        collider_set,
                        &poly,
                        &mut terrain_handles,
                    );

                    counter += 2;
                    original_y = y;
                }

                TerrainState::Pit if !oneshot => {
                    y = original_y;
                    if counter > 1 {
                        y -= 4.0 * self.config.terrain_step;
                    }
                }

                TerrainState::Stump if oneshot => {
                    counter = rng.random_range(1i32..3i32);
                    let step = self.config.terrain_step;
                    let poly = vec![
                        Vector2::new(x, y),
                        Vector2::new(x + counter as f32 * step, y),
                        Vector2::new(x + counter as f32 * step, y + counter as f32 * step),
                        Vector2::new(x, y + counter as f32 * step),
                    ];
                    self.create_polygon_terrain(
                        rigid_body_set,
                        collider_set,
                        &poly,
                        &mut terrain_handles,
                    );
                }

                TerrainState::Stairs if oneshot => {
                    stair_height = if rng.random_bool(0.5) { 1 } else { -1 };
                    stair_width = rng.random_range(4i32..5i32);
                    stair_steps = rng.random_range(3i32..5i32);
                    original_y = y;

                    let step = self.config.terrain_step;
                    for s in 0..stair_steps {
                        let poly = vec![
                            Vector2::new(
                                x + (s * stair_width) as f32 * step,
                                y + (s * stair_height) as f32 * step,
                            ),
                            Vector2::new(
                                x + ((1 + s) * stair_width) as f32 * step,
                                y + (s * stair_height) as f32 * step,
                            ),
                            Vector2::new(
                                x + ((1 + s) * stair_width) as f32 * step,
                                y + (-1 + s * stair_height) as f32 * step,
                            ),
                            Vector2::new(
                                x + (s * stair_width) as f32 * step,
                                y + (-1 + s * stair_height) as f32 * step,
                            ),
                        ];
                        self.create_polygon_terrain(
                            rigid_body_set,
                            collider_set,
                            &poly,
                            &mut terrain_handles,
                        );
                    }
                    counter = stair_steps * stair_width;
                }

                TerrainState::Stairs if !oneshot => {
                    let s = (stair_steps * stair_width) - counter - stair_height;
                    let n = s as f32 / stair_width as f32;
                    y = original_y + n * stair_height as f32 * self.config.terrain_step;
                }

                _ => {}
            }

            oneshot = false;
            terrain_y.push(y);
            counter = if counter > 0 { counter - 1 } else { 0 };

            if counter == 0 {
                counter = rng.random_range(
                    (self.config.terrain_grass / 2) as i32..self.config.terrain_grass as i32,
                );
                if state == TerrainState::Grass && hardcore {
                    state = match rng.random_range(1..4) {
                        1 => TerrainState::Stump,
                        2 => TerrainState::Stairs,
                        3 => TerrainState::Pit,
                        _ => TerrainState::Grass,
                    };
                    oneshot = true;
                } else {
                    state = TerrainState::Grass;
                    oneshot = true;
                }
            }
        }

        // Create terrain edges (the actual walking surface)
        for i in 0..(self.config.terrain_length - 1) {
            let p1 = Point2::new(terrain_x[i], terrain_y[i]);
            let p2 = Point2::new(terrain_x[i + 1], terrain_y[i + 1]);

            let rigid_body = RigidBodyBuilder::fixed()
                .translation(vector![0.0, 0.0])
                .build();
            let handle = rigid_body_set.insert(rigid_body);

            let collider = ColliderBuilder::segment(p1, p2)
                .friction(self.config.friction)
                .active_events(ActiveEvents::COLLISION_EVENTS)
                .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::ALL))
                .build();
            collider_set.insert_with_parent(collider, handle, rigid_body_set);

            terrain_handles.push(handle);
        }

        (terrain_handles, terrain_x, terrain_y)
    }

    fn create_polygon_terrain(
        &self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        vertices: &[Vector2<f32>],
        handles: &mut Vec<RigidBodyHandle>,
    ) {
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(vector![0.0, 0.0])
            .build();
        let handle = rigid_body_set.insert(rigid_body);

        let points: Vec<Point2<f32>> = vertices.iter().map(|v| Point2::new(v.x, v.y)).collect();
        let collider = ColliderBuilder::convex_hull(&points)
            .unwrap()
            .friction(self.config.friction)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        collider_set.insert_with_parent(collider, handle, rigid_body_set);

        handles.push(handle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_terrain_generator_creation() {
        let config = BipedalWalkerConfig::default();
        let generator = TerrainGenerator::new(config.clone());

        // Since generator just holds config, we verify indirectly via success
        assert_eq!(generator.config.terrain_length, 200);
    }

    #[test]
    fn test_generate_basic_structure() {
        let config = BipedalWalkerConfig::default();
        let generator = TerrainGenerator::new(config);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut rng = StepRng::new(0, 1);

        let (handles, x_coords, y_coords) =
            generator.generate(&mut rng, &mut rigid_body_set, &mut collider_set, false);

        assert!(!handles.is_empty());
        assert_eq!(x_coords.len(), 200);
        assert_eq!(y_coords.len(), 200);

        assert!(!rigid_body_set.is_empty());
        assert!(!collider_set.is_empty());

        assert!(handles.len() >= 199);
    }

    #[test]
    fn test_generate_hardcore_mode() {
        let config = BipedalWalkerConfig::default();
        let generator = TerrainGenerator::new(config);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut rng = StepRng::new(42, 1); // Different seed to encourage feature generation

        let (handles, _, _) = generator.generate(
            &mut rng,
            &mut rigid_body_set,
            &mut collider_set,
            true, // Hardcore enabled
        );

        assert!(!handles.is_empty());
        assert!(!rigid_body_set.is_empty());
    }

    #[test]
    fn test_coordinate_monotonicity() {
        let config = BipedalWalkerConfig::default();
        let generator = TerrainGenerator::new(config.clone());

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut rng = StepRng::new(0, 1);

        let (_, x_coords, _) =
            generator.generate(&mut rng, &mut rigid_body_set, &mut collider_set, false);

        for i in 0..x_coords.len() - 1 {
            assert!(x_coords[i + 1] > x_coords[i]);
            let step = x_coords[i + 1] - x_coords[i];
            assert!((step - config.terrain_step).abs() < 1e-5);
        }
    }

    #[test]
    fn test_create_polygon_terrain() {
        let config = BipedalWalkerConfig::default();
        let generator = TerrainGenerator::new(config);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut handles = Vec::new();

        let poly = vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ];

        generator.create_polygon_terrain(
            &mut rigid_body_set,
            &mut collider_set,
            &poly,
            &mut handles,
        );

        assert_eq!(handles.len(), 1);
        assert_eq!(rigid_body_set.len(), 1);
        assert_eq!(collider_set.len(), 1);

        let handle = handles[0];
        let rb = rigid_body_set.get(handle).unwrap();
        assert!(rb.is_fixed());
    }

    #[test]
    fn test_startpad_flatness() {
        let config = BipedalWalkerConfig::default();
        let startpad_len = config.terrain_startpad;
        let generator = TerrainGenerator::new(config);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut rng = StepRng::new(123, 1);

        let (_, _, y_coords) =
            generator.generate(&mut rng, &mut rigid_body_set, &mut collider_set, false);

        let initial_y = y_coords[0];
        for y in y_coords.iter().take(startpad_len).skip(1) {
            assert!((*y - initial_y).abs() < 5.0);
        }
    }
}
