use nalgebra::{Point2, Vector2};
use rand::Rng;
use rapier2d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerrainState {
    Grass,
    Stump,
    Stairs,
    Pit,
}

pub struct TerrainGenerator {
    config: super::config::BipedalWalkerConfig,
}

impl TerrainGenerator {
    pub fn new(config: super::config::BipedalWalkerConfig) -> Self {
        Self { config }
    }

    pub fn generate<R: Rng>(
        &self,
        rng: &mut R,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        hardcore: bool,
    ) -> (Vec<RigidBodyHandle>, Vec<f64>, Vec<f64>) {
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
            let x = i as f64 * self.config.terrain_step;
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
                        Vector2::new(x as f32, y as f32),
                        Vector2::new((x + step) as f32, y as f32),
                        Vector2::new((x + step) as f32, (y - 4.0 * step) as f32),
                        Vector2::new(x as f32, (y - 4.0 * step) as f32),
                    ];
                    self.create_polygon_terrain(
                        rigid_body_set,
                        collider_set,
                        &poly,
                        &mut terrain_handles,
                    );

                    // Create right wall
                    let offset = step * counter as f64;
                    let poly = vec![
                        Vector2::new((x + offset) as f32, y as f32),
                        Vector2::new((x + offset + step) as f32, y as f32),
                        Vector2::new((x + offset + step) as f32, (y - 4.0 * step) as f32),
                        Vector2::new((x + offset) as f32, (y - 4.0 * step) as f32),
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
                        Vector2::new(x as f32, y as f32),
                        Vector2::new((x + counter as f64 * step) as f32, y as f32),
                        Vector2::new(
                            (x + counter as f64 * step) as f32,
                            (y + counter as f64 * step) as f32,
                        ),
                        Vector2::new(x as f32, (y + counter as f64 * step) as f32),
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
                                (x + (s * stair_width) as f64 * step) as f32,
                                (y + (s * stair_height) as f64 * step) as f32,
                            ),
                            Vector2::new(
                                (x + ((1 + s) * stair_width) as f64 * step) as f32,
                                (y + (s * stair_height) as f64 * step) as f32,
                            ),
                            Vector2::new(
                                (x + ((1 + s) * stair_width) as f64 * step) as f32,
                                (y + (-1 + s * stair_height) as f64 * step) as f32,
                            ),
                            Vector2::new(
                                (x + (s * stair_width) as f64 * step) as f32,
                                (y + (-1 + s * stair_height) as f64 * step) as f32,
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
                    let s = (stair_steps * stair_width) as i32 - counter as i32 - stair_height;
                    let n = s as f64 / stair_width as f64;
                    y = original_y + n * stair_height as f64 * self.config.terrain_step;
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
            let p1 = Point2::new(terrain_x[i] as f32, terrain_y[i] as f32);
            let p2 = Point2::new(terrain_x[i + 1] as f32, terrain_y[i + 1] as f32);

            let rigid_body = RigidBodyBuilder::fixed()
                .translation(vector![0.0, 0.0])
                .build();
            let handle = rigid_body_set.insert(rigid_body);

            let collider = ColliderBuilder::segment(p1, p2)
                .friction(self.config.friction as f32)
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

        // Convert Vector2 to Point2 for convex hull
        let points: Vec<Point2<f32>> = vertices.iter().map(|v| Point2::new(v.x, v.y)).collect();
        let collider = ColliderBuilder::convex_hull(&points)
            .unwrap()
            .friction(self.config.friction as f32)
            .build();
        collider_set.insert_with_parent(collider, handle, rigid_body_set);

        handles.push(handle);
    }
}
