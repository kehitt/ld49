use super::{
    component::{Transform, Velocity},
    resource::{DeltaTime, GameState, GameStateForRenderer},
};
use specs::prelude::*;

mod asteroid_spawner_system;
mod entity_lifetime_system;
mod entity_spinner_system;
mod game_manager_system;
mod particle_spawner_system;
mod player_bounds_enforcer_system;
mod player_collision_system;
mod player_movement_system;
mod render_system;
mod repair_pack_manager_system;

pub use asteroid_spawner_system::AsteroidSpawnerSystem;
pub use entity_lifetime_system::EntityLifetimeSystem;
pub use entity_spinner_system::EntitySpinnerSystem;
pub use game_manager_system::GameManagerSystem;
pub use particle_spawner_system::ParticleSpawnerSystem;
pub use player_bounds_enforcer_system::PlayerBoundsEnforcerSystem;
pub use player_collision_system::PlayerCollisionSystem;
pub use player_movement_system::PlayerMovementSystem;
pub use render_system::RenderSystem;
pub use repair_pack_manager_system::RepairPackManagerSystem;

#[derive(Default)]
pub struct VelocityApplicatorSystem {}

impl<'a> System<'a> for VelocityApplicatorSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Transform>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (velocity, transform) in (&vel, &mut pos).join() {
            transform.position += velocity.direction * velocity.speed;
        }
    }
}

pub struct BackgroundAnimatorSystem {
    anim_speed: f32,
    anim_clock: f32,
}

impl Default for BackgroundAnimatorSystem {
    fn default() -> Self {
        Self {
            anim_speed: 0.1,
            anim_clock: 0.0,
        }
    }
}

impl<'a> System<'a> for BackgroundAnimatorSystem {
    type SystemData = (
        Read<'a, GameState>,
        Write<'a, GameStateForRenderer>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (game_state, mut game_state_renderer, dt): Self::SystemData) {
        match *game_state {
            GameState::GameStateInit {} => {
                game_state_renderer.background_idx = 0;
            }
            GameState::GameStatePlay { .. } => {
                self.anim_clock -= dt.0.as_secs_f32();

                if self.anim_clock < 0.0 {
                    game_state_renderer.background_idx += 1;

                    if game_state_renderer.background_idx < 2
                        || game_state_renderer.background_idx > 4
                    {
                        game_state_renderer.background_idx = 2;
                    }

                    self.anim_clock = self.anim_speed
                }
            }
            GameState::GameStateEnd {} => {
                game_state_renderer.background_idx = 1;
            }
        }
    }
}
