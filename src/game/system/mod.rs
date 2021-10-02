use specs::prelude::*;

use super::{
    component::{Transform, Velocity},
    resource::DeltaTime,
};

mod render_system;
pub use render_system::RenderSystem;

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Transform>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {}
    }
}

#[derive(Default)]
pub struct UpdatePos {
    angle: f32,
}

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        let delta = delta.0.as_secs_f32();

        self.angle += delta;

        for (velocity, transform) in (&vel, &mut pos).join() {
            transform.rot = glam::Quat::from_rotation_z(self.angle);
        }
    }
}
