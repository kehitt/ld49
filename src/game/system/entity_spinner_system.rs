use glam::EulerRot;
use specs::prelude::*;

use crate::game::{
    component::{Spinner, Transform},
    resource::DeltaTime,
};

#[derive(Default)]
pub struct EntitySpinnerSystem {}

impl<'a> System<'a> for EntitySpinnerSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Spinner>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut trans, spin, dt): Self::SystemData) {
        for (transform, spinner) in (&mut trans, &spin).join() {
            let (_, _, mut angle) = transform.rotation.to_euler(EulerRot::XYZ);
            angle += spinner.speed * dt.0.as_secs_f32();
            transform.rotation = glam::Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
        }
    }
}
