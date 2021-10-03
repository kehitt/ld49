use specs::{prelude::*, shrev::EventChannel};
use winit::dpi::PhysicalSize;

use crate::{
    game::component::{Display, Transform},
    game::resource::WindowEvent,
    renderer::SpriteRenderer,
};

#[derive(Default)]
pub struct RenderSystem {
    renderer: Option<SpriteRenderer>,
    reader: Option<ReaderId<WindowEvent>>,
}

impl RenderSystem {
    pub fn new(renderer: SpriteRenderer) -> Self {
        Self {
            renderer: Some(renderer),
            reader: None,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Display>,
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

    fn run(&mut self, (pos, disp, events): Self::SystemData) {
        if let Some(renderer) = &mut self.renderer {
            // Process events
            for event in events.read(&mut self.reader.as_mut().unwrap()) {
                #[allow(unreachable_patterns)]
                match event {
                    WindowEvent::Resize(new_width, new_height) => {
                        renderer.on_resize(PhysicalSize::new(*new_width, *new_height))
                    }
                    _ => (),
                }
            }
            // Render stuff
            for (position, display) in (&pos, &disp).join() {
                renderer.add_sprite_instance(display.sprite_idx, position.to_model_mat())
            }
            renderer.draw_instances_or_panic();
        } else {
            panic!("No renderer was set!")
        }
    }
}
