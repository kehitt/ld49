use std::time::Duration;

use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use specs::{shrev::EventChannel, Dispatcher, DispatcherBuilder, World, WorldExt};

use crate::{
    game::{
        resource::{DeltaTime, GameWindowSize, KeyboardEvent, WindowEvent as GameWindowEvent},
        system::{
            AsteroidSpawnerSystem, EntityLifetimeSystem, GameManagerSystem,
            PlayerBoundsEnforcerSystem, PlayerCollisionSystem, PlayerMovementSystem, RenderSystem,
            RepairPackManagerSystem, VelocityApplicatorSystem,
        },
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
        let size = window.inner_size();
        let mut world = World::new();
        let mut update_dispatcher = DispatcherBuilder::new()
            .with(GameManagerSystem::default(), "game_manager_system", &[])
            .with(
                AsteroidSpawnerSystem::default(),
                "asteroid_spawner_system",
                &["game_manager_system"],
            )
            .with(
                PlayerMovementSystem::default(),
                "player_movement_system",
                &["game_manager_system"],
            )
            .with(
                PlayerCollisionSystem::default(),
                "player_collision_system",
                &["game_manager_system"],
            )
            .with(
                VelocityApplicatorSystem::default(),
                "velocity_applicator",
                &[],
            )
            .with(PlayerBoundsEnforcerSystem::default(), "bounds_keeper", &[])
            .with(
                EntityLifetimeSystem::default(),
                "entity_lifetime_system",
                &[],
            )
            .with(
                RepairPackManagerSystem::default(),
                "repair_pack_manager_system",
                &[],
            )
            .build();

        let sprite_atlas_bytes = include_bytes!("../assets/spritesheet.png");

        let mut render_dispatcher = DispatcherBuilder::new()
            .with(
                RenderSystem::new(pollster::block_on(SpriteRenderer::new(
                    window,
                    sprite_atlas_bytes,
                    (64, 64),
                ))),
                "render_system",
                &[],
            )
            .build();

        update_dispatcher.setup(&mut world);
        render_dispatcher.setup(&mut world);

        {
            // Setup for the first time, since render system is not called before all the updates
            let mut game_window_size = world.write_resource::<GameWindowSize>();
            *game_window_size = GameWindowSize(size.width, size.height);
        }

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
                WindowEvent::Resized(physical_size) => self
                    .world
                    .fetch_mut::<EventChannel<GameWindowEvent>>()
                    .single_write(GameWindowEvent::Resize(
                        physical_size.width,
                        physical_size.height,
                    )),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self
                    .world
                    .fetch_mut::<EventChannel<GameWindowEvent>>()
                    .single_write(GameWindowEvent::Resize(
                        new_inner_size.width,
                        new_inner_size.height,
                    )),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: element_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    let event = match element_state {
                        ElementState::Pressed => KeyboardEvent::Pressed(keycode),
                        ElementState::Released => KeyboardEvent::Released(keycode),
                    };

                    self.world
                        .fetch_mut::<EventChannel<KeyboardEvent>>()
                        .single_write(event)
                }
                _ => (),
            },
            _ => (),
        }
    }
}
