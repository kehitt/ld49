use super::component::{Transform, Velocity};
use specs::prelude::*;

mod asteroid_spawner_system;
mod entity_lifetime_system;
mod game_manager_system;
mod player_bounds_enforcer_system;
mod player_collision_system;
mod player_movement_system;
mod render_system;

pub use asteroid_spawner_system::AsteroidSpawnerSystem;
pub use entity_lifetime_system::EntityLifetimeSystem;
pub use game_manager_system::GameManagerSystem;
pub use player_bounds_enforcer_system::PlayerBoundsEnforcerSystem;
pub use player_collision_system::PlayerCollisionSystem;
pub use player_movement_system::PlayerMovementSystem;
pub use render_system::RenderSystem;

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
