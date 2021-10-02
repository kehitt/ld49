use std::time::{Duration, Instant};

use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

use ld49::app::App;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Game")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(&window);

    let mut previous_tick = Instant::now();
    let mut lag = 0_u32;
    let ms_per_update = 1000 / 50; // 50 ticks per second

    app.on_setup();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                let elapsed_time = previous_tick.elapsed().as_millis() as u32;
                previous_tick = Instant::now();
                lag += elapsed_time;

                while lag >= ms_per_update {
                    if let Some(new_control_flow) =
                        app.on_update(Duration::from_millis(ms_per_update.into()))
                    {
                        *control_flow = new_control_flow;
                    }
                    lag -= ms_per_update;
                }

                window.request_redraw();
            }
            Event::RedrawRequested(_) => app.on_render(f64::from(lag) / f64::from(ms_per_update)),
            event => app.on_event(event),
        };
    });
}
