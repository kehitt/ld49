use rand::Rng;
use specs::prelude::*;

use crate::game::{
    component::{Collider, ColliderTag, Display, Lifetime, Spinner, Transform, Velocity},
    resource::{DeltaTime, GameState, GameWindowSize},
};

pub struct AsteroidSpawnerSystem {
    spawn_timeout: f32,
    spawn_clock: f32,
}

impl Default for AsteroidSpawnerSystem {
    fn default() -> Self {
        Self {
            spawn_timeout: 1.0,
            spawn_clock: 0.0,
        }
    }
}

impl<'a> System<'a> for AsteroidSpawnerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, GameState>,
        Read<'a, DeltaTime>,
        Read<'a, GameWindowSize>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (entities, updater, game_state, dt, game_window_size): Self::SystemData) {
        match *game_state {
            GameState::GameStatePlay { .. } => {
                self.spawn_clock -= dt.0.as_secs_f32();

                if self.spawn_clock < 0.0 {
                    self.spawn_asteroid(&game_window_size, &entities, &updater);
                    self.spawn_clock = self.spawn_timeout
                }
            }
            _ => (),
        }
    }
}

impl AsteroidSpawnerSystem {
    fn spawn_asteroid<'a>(
        &self,
        game_window_size: &GameWindowSize,
        entities: &Entities,
        updater: &Read<'a, LazyUpdate>,
    ) {
        let asteroid = entities.create();
        let mut rng = rand::thread_rng();

        let max_random_speed = 5.0;

        let transform_pos = glam::vec2(
            rng.gen_range(-(game_window_size.0 as f32 / 2.0)..(game_window_size.0 as f32 / 2.0)),
            game_window_size.1 as f32,
        );

        let velocity_dir = glam::vec2(rng.gen_range(-0.1..0.1), -1.0).normalize();
        let velocity_speed = rng.gen_range(1.0..max_random_speed);

        let scale = glam::vec2(
            100.0 - ((max_random_speed - velocity_speed) * 10.0),
            100.0 - ((max_random_speed - velocity_speed) * 10.0),
        );

        let sprite_idx = rng.gen_range(2..=4);
        let speen_speed = rng.gen_range(0.1..5.0);

        updater.insert(
            asteroid,
            Transform {
                position: transform_pos,
                scale,
                ..Default::default()
            },
        );
        updater.insert(
            asteroid,
            Velocity {
                direction: velocity_dir,
                speed: velocity_speed,
            },
        );
        updater.insert(asteroid, Display { sprite_idx });
        updater.insert(asteroid, Spinner { speed: speen_speed });
        updater.insert(asteroid, Collider::new(ColliderTag::Asteroid));
        updater.insert(
            asteroid,
            Lifetime {
                remaining: 10.0 * velocity_speed,
            },
        );
    }
}
