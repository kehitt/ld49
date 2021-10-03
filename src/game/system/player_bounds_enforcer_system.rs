use specs::prelude::*;

use crate::game::{
    component::{Player, Transform, Velocity},
    resource::GameWindowSize,
};

#[derive(Default)]
pub struct PlayerBoundsEnforcerSystem;

impl<'a> System<'a> for PlayerBoundsEnforcerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        Read<'a, GameWindowSize>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (player, mut vel, mut pos, game_window_size): Self::SystemData) {
        for (_, velocity, transform) in (&player, &mut vel, &mut pos).join() {
            let half_width = (game_window_size.0 / 2) as f32;
            let half_height = (game_window_size.1 / 2) as f32;

            if transform.position.x > half_width || transform.position.x < -half_width {
                velocity.direction.x = -velocity.direction.x;
            } else if transform.position.y > half_height || transform.position.y < -half_height {
                velocity.direction.y = -velocity.direction.y;
            }

            transform.position = transform.position.clamp(
                glam::vec2(-half_width, -half_height),
                glam::vec2(half_width, half_height),
            )
        }
    }
}
