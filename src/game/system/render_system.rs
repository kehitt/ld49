use specs::prelude::*;

use crate::{
    game::component::{Display, Transform},
    renderer::SpriteRenderer,
};

#[derive(Default)]
pub struct RenderSystem {
    pub renderer: Option<SpriteRenderer>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Display>);

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (pos, disp): Self::SystemData) {
        if let Some(renderer) = &mut self.renderer {
            for (position, display) in (&pos, &disp).join() {
                renderer.add_sprite_instance(display.sprite_idx, position.to_model_mat())
            }
            renderer.draw_instances_or_panic();
        } else {
            panic!("No renderer was set!")
        }
    }
}
