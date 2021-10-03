use specs::prelude::*;

use crate::game::{
    component::{Collider, ColliderTag, Player, Transform},
    resource::DeltaTime,
};

#[derive(Default)]
pub struct PlayerCollisionSystem;

impl<'a> System<'a> for PlayerCollisionSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
        Read<'a, DeltaTime>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (mut player, tf, coll, dt): Self::SystemData) {
        for (player_comp, player_transform, player_collider) in (&mut player, &tf, &coll).join() {
            for (other_transform, other_collider) in (&tf, &coll).join() {
                if player_collider == other_collider {
                    // Skip self
                    continue;
                }

                // Unholy, probably want to cache it with flagged storage
                if player_collider
                    .bounding_box
                    .translate(player_transform.position)
                    .scale(player_transform.scale)
                    .intersect(
                        &other_collider
                            .bounding_box
                            .translate(other_transform.position)
                            .scale(other_transform.scale),
                    )
                {
                    if let ColliderTag::Asteroid = other_collider.tag {
                        player_comp.health -= 50.0 * dt.0.as_secs_f32();
                        println!("Player health: {}", player_comp.health);
                    }
                }
            }
        }
    }
}
