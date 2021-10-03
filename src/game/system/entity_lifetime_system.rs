use specs::prelude::*;

use crate::game::{component::Lifetime, resource::DeltaTime};

#[derive(Default)]
pub struct EntityLifetimeSystem {}

impl<'a> System<'a> for EntityLifetimeSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Lifetime>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut lifetime, dt): Self::SystemData) {
        for (entity, lifetime_component) in (&entities, &mut lifetime).join() {
            lifetime_component.remaining -= dt.0.as_secs_f32();

            if lifetime_component.remaining <= 0.0 {
                entities.delete(entity).unwrap()
            }
        }
    }
}
