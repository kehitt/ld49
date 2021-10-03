use specs::{prelude::*, shrev::EventChannel};
use winit::event::VirtualKeyCode;

use crate::game::{
    component::{Collider, ColliderTag, Display, Player, Transform, Velocity},
    resource::{DeltaTime, GameState, GameStateForRenderer, KeyboardEvent},
};

const GAMERULE_PLAYER_TICK_DAMAGE: f32 = 4.8;

#[derive(Default)]
pub struct GameManagerSystem {
    reader: Option<ReaderId<KeyboardEvent>>,
}

impl<'a> System<'a> for GameManagerSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, GameState>,
        Write<'a, GameStateForRenderer>,
        Read<'a, DeltaTime>,
        Read<'a, EventChannel<KeyboardEvent>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(
            world
                .fetch_mut::<EventChannel<KeyboardEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            mut player_storage,
            entities,
            updater,
            mut game_state,
            mut game_state_renderer,
            dt,
            events,
        ): Self::SystemData,
    ) {
        match *game_state {
            GameState::GameStateInit {} => {
                // @REFACTOR
                for event in events.read(&mut self.reader.as_mut().unwrap()) {
                    match event {
                        &KeyboardEvent::Pressed(VirtualKeyCode::Return) => {
                            let player_entity = spawn_player(&entities, &updater);
                            *game_state = GameState::GameStatePlay { player_entity };
                        }
                        _ => (),
                    }
                }
            }
            GameState::GameStatePlay { player_entity } => {
                let player_component = player_storage.get_mut(player_entity).unwrap();
                player_component.health -= GAMERULE_PLAYER_TICK_DAMAGE * dt.0.as_secs_f32();
                if player_component.health <= 0.0 {
                    entities.delete(player_entity).unwrap();
                    *game_state = GameState::GameStateEnd {};
                }
                game_state_renderer.player_health = player_component.health / 100.0;
            }
            GameState::GameStateEnd {} => {
                // @REFACTOR
                for event in events.read(&mut self.reader.as_mut().unwrap()) {
                    match event {
                        &KeyboardEvent::Pressed(VirtualKeyCode::Return) => {
                            *game_state = GameState::GameStateInit {};
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn spawn_player<'a>(entities: &Entities, updater: &Read<'a, LazyUpdate>) -> Entity {
    let player = entities.create();
    updater.insert(player, Transform::default());
    updater.insert(player, Velocity::default());
    updater.insert(player, Display { sprite_idx: 0 });
    updater.insert(player, Player::default());
    updater.insert(player, Collider::new(ColliderTag::Player));

    player
}
