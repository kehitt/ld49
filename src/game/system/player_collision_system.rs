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
                    match other_collider.tag {
                        ColliderTag::Player => (),
                        ColliderTag::Asteroid => {
                            player_comp.health -= 50.0 * dt.0.as_secs_f32();
                        }
                        ColliderTag::Health => {
                            player_comp.health += 30.0;
                            if player_comp.health > 100.0 {
                                player_comp.health = 100.0;
                            }
                        }
                    }
                }
            }
        }
    }
}
