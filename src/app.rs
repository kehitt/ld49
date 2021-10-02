use std::time::Duration;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

use crate::{
    game::{
        component::{Display, Transform, Velocity},
        resource::DeltaTime,
        system::{HelloWorld, RenderSystem, UpdatePos},
    },
    renderer::SpriteRenderer,
};

pub struct App<'a> {
    world: World,
    update_dispatcher: Dispatcher<'a, 'a>,
    render_dispatcher: Dispatcher<'a, 'a>,
    close_requested: bool,
}

impl<'a> App<'_> {
    pub fn new(window: &Window) -> Self {
        let mut world = World::new();
        let mut update_dispatcher = DispatcherBuilder::new()
            .with(UpdatePos::default(), "update_pos", &[])
            .with(HelloWorld, "hello_updated", &["update_pos"])
            .build();

        let mut render_dispatcher = DispatcherBuilder::new()
            .with(
                RenderSystem {
                    renderer: Some(pollster::block_on(SpriteRenderer::new(window))),
                },
                "render_system",
                &[],
            )
            .build();

        update_dispatcher.setup(&mut world);
        render_dispatcher.setup(&mut world);

        world
            .create_entity()
            .with(Transform {
                pos: glam::Vec3::new(1.0, 0.0, -10.0),
                rot: glam::Quat::IDENTITY,
            })
            .with(Velocity { x: 1.0, y: -1.0 })
            .with(Display { sprite_idx: 0 })
            .build();

        world
            .create_entity()
            .with(Transform {
                pos: glam::Vec3::new(0.0, 1.0, -10.0),
                rot: glam::Quat::IDENTITY,
            })
            .with(Velocity { x: 1.0, y: 1.0 })
            .with(Display { sprite_idx: 0 })
            .build();

        world
            .create_entity()
            .with(Transform {
                pos: glam::Vec3::new(0.0, 0.0, -10.0),
                rot: glam::Quat::IDENTITY,
            })
            .with(Velocity { x: -1.0, y: -1.0 })
            .with(Display { sprite_idx: 0 })
            .build();

        Self {
            world: world,
            update_dispatcher,
            render_dispatcher,
            close_requested: false,
        }
    }

    pub fn on_update(&mut self, delta_time: Duration) -> Option<ControlFlow> {
        if self.close_requested {
            return Some(ControlFlow::Exit);
        }

        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(delta_time);
        }

        self.update_dispatcher.dispatch(&mut self.world);
        self.world.maintain();

        None
    }

    pub fn on_render(&mut self, _interpolation: f64) {
        // @FIXME If render is too quick than we never update
        self.render_dispatcher.dispatch(&mut self.world);
    }

    pub fn on_event(&mut self, event: Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => self.close_requested = true,
                WindowEvent::Resized(physical_size) => {
                    // Dispatch world events
                    //self.renderer.on_resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // Dispatch world events
                    //self.renderer.on_resize(*new_inner_size);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
