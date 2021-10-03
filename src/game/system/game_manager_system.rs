use specs::prelude::*;

use crate::game::{
    component::{Collider, ColliderTag, Display, Player, Transform, Velocity},
    resource::GameState,
};

#[derive(Default)]
pub struct GameManagerSystem;

impl<'a> System<'a> for GameManagerSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, GameState>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (mut player_storage, entities, updater, mut game_state): Self::SystemData) {
        match *game_state {
            GameState::GameStateInit {} => {
                spawn_player(&entities, &updater);
                *game_state = GameState::GameStatePlay {};
            }
            GameState::GameStatePlay {} => {
                for (entity, player) in (&entities, &mut player_storage).join() {
                    if player.health <= 0.0 {
                        entities.delete(entity).unwrap();
                        *game_state = GameState::GameStateEnd {};
                    }
                }
            }
            GameState::GameStateEnd {} => {
                println!("Game ended!");
            }
        }
    }
}

fn spawn_player<'a>(entities: &Entities, updater: &Read<'a, LazyUpdate>) {
    let player = entities.create();
    updater.insert(player, Transform::default());
    updater.insert(player, Velocity::default());
    updater.insert(player, Display { sprite_idx: 0 });
    updater.insert(player, Player::default());
    updater.insert(player, Collider::new(ColliderTag::Player));
}
