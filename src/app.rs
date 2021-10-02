use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::renderer::Renderer;

pub struct App {
    renderer: Renderer,
    close_requested: bool,
}

impl App {
    pub fn new(window: &Window) -> Self {
        Self {
            renderer: pollster::block_on(Renderer::new(window)),
            close_requested: false,
        }
    }

    pub fn on_update(&self) -> Option<ControlFlow> {
        if self.close_requested {
            return Some(ControlFlow::Exit);
        }

        None
    }

    pub fn on_render(&mut self, _interpolation: f64) {
        // @FIXME If render is too quick than we never update
        self.renderer.on_draw();
    }

    pub fn on_event(&mut self, event: Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => self.close_requested = true,
                WindowEvent::Resized(physical_size) => {
                    self.renderer.on_resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.renderer.on_resize(*new_inner_size);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
