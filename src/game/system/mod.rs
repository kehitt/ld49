use specs::prelude::*;
use specs::shrev::EventChannel;

use super::component::{Transform, Velocity};
use super::resource::WindowEvent;

mod player_collision_system;
mod player_movement_system;
mod render_system;

pub use player_collision_system::PlayerCollisionSystem;
pub use player_movement_system::PlayerMovementSystem;
pub use render_system::RenderSystem;

#[derive(Default)]
pub struct VelocityApplicator {}

impl<'a> System<'a> for VelocityApplicator {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Transform>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (velocity, transform) in (&vel, &mut pos).join() {
            transform.position += velocity.direction * velocity.speed;
        }
    }
}

#[derive(Default)]
pub struct ScreenBoundsKeeper {
    bounds: (u32, u32),
    reader: Option<ReaderId<WindowEvent>>,
}

impl ScreenBoundsKeeper {
    pub fn new(init_bounds: (u32, u32)) -> Self {
        Self {
            bounds: init_bounds,
            reader: None,
        }
    }
}

impl<'a> System<'a> for ScreenBoundsKeeper {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        Read<'a, EventChannel<WindowEvent>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(
            world
                .fetch_mut::<EventChannel<WindowEvent>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (mut vel, mut pos, events): Self::SystemData) {
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            #[allow(unreachable_patterns)]
            match event {
                WindowEvent::Resize(new_width, new_height) => {
                    self.bounds = (*new_width, *new_height);
                }
                _ => (),
            }
        }

        for (velocity, transform) in (&mut vel, &mut pos).join() {
            let half_width = (self.bounds.0 / 2) as f32;
            let half_height = (self.bounds.1 / 2) as f32;

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
