use rand::Rng;
use specs::prelude::*;

use crate::game::{
    component::{Collider, ColliderTag, Display, Player, Transform},
    resource::{DeltaTime, GameState, GameWindowSize},
};

pub struct RepairPackManagerSystem {
    spawn_timeout: f32,
    spawn_clock: f32,
    active_entity: Option<Entity>,
}

impl Default for RepairPackManagerSystem {
    fn default() -> Self {
        Self {
            spawn_timeout: 5.0,
            spawn_clock: 0.0,
            active_entity: None,
        }
    }
}

impl<'a> System<'a> for RepairPackManagerSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
        Read<'a, LazyUpdate>,
        Read<'a, GameState>,
        Read<'a, DeltaTime>,
        Read<'a, GameWindowSize>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(
        &mut self,
        (
            entities,
            player_storage,
            transform_storage,
            collider_storage,
            updater,
            game_state,
            dt,
            game_window_size,
        ): Self::SystemData,
    ) {
        match *game_state {
            GameState::GameStatePlay { .. } => {
                self.spawn_clock -= dt.0.as_secs_f32();
                let mut entity_deleted = false;

                if self.spawn_clock < 0.0 {
                    if let Some(active_pack) = &self.active_entity {
                        entities.delete(*active_pack).unwrap();
                        self.active_entity = None;
                    }
                    self.active_entity =
                        Some(self.spawn_health_pack(&game_window_size, &entities, &updater));
                    self.spawn_clock = self.spawn_timeout
                } else if let Some(active_pack) = &self.active_entity {
                    // Check if any players have collided with the pack to destroy it
                    // @REFACTOR Definitely not the best solution (im thinking generic pickup system)
                    let repair_pack_collider = collider_storage.get(*active_pack).unwrap();
                    let repair_pack_transform = transform_storage.get(*active_pack).unwrap();

                    for (_, player_transform, player_collider) in
                        (&player_storage, &transform_storage, &collider_storage).join()
                    {
                        if player_collider
                            .bounding_box
                            .translate(player_transform.position)
                            .scale(player_transform.scale)
                            .intersect(
                                &repair_pack_collider
                                    .bounding_box
                                    .translate(repair_pack_transform.position)
                                    .scale(repair_pack_transform.scale),
                            )
                        {
                            entities.delete(*active_pack).unwrap();
                            entity_deleted = true;
                        }
                    }
                }

                if entity_deleted {
                    self.active_entity = None
                }
            }
            GameState::GameStateEnd {} => {
                if let Some(active_pack) = self.active_entity.take() {
                    entities.delete(active_pack).unwrap();
                }
            }
            _ => (),
        }
    }
}

impl RepairPackManagerSystem {
    fn spawn_health_pack<'a>(
        &self,
        game_window_size: &GameWindowSize,
        entities: &Entities,
        updater: &Read<'a, LazyUpdate>,
    ) -> Entity {
        let asteroid = entities.create();
        let mut rng = rand::thread_rng();

        let transform_pos = glam::vec2(
            rng.gen_range(-(game_window_size.0 as f32 / 3.0)..(game_window_size.0 as f32 / 3.0)),
            rng.gen_range(-(game_window_size.1 as f32 / 3.0)..(game_window_size.1 as f32 / 3.0)),
        );

        updater.insert(
            asteroid,
            Transform {
                position: transform_pos,
                scale: glam::vec2(40.0, 40.0),
                ..Default::default()
            },
        );
        updater.insert(asteroid, Display { sprite_idx: 1 });
        updater.insert(asteroid, Collider::new(ColliderTag::Health));

        asteroid
    }
}
