use specs::prelude::*;

use crate::{
    game::component::{Collider, Player, Transform}
};

#[derive(Default)]
pub struct PlayerCollisionSystem;

impl<'a> System<'a> for PlayerCollisionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (player, tf, coll): Self::SystemData) {
        for (_, player_transform, player_collider) in (&player, &tf, &coll).join() {
            for ((), other_transform, other_collider) in (!&player, &tf, &coll).join() {
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
                    println!("Player collided with {:?}!", other_collider.tag);
                }
            }
        }
    }
}
