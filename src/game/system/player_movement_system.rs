use glam::EulerRot;
use specs::{prelude::*, shred::DefaultProvider, shrev::EventChannel};
use winit::event::VirtualKeyCode;

use crate::{
    game::component::{Transform, Velocity},
    game::{
        component::Player,
        resource::{DeltaTime, KeyboardEvent},
    },
};

const MAX_PLAYER_SPEED: f32 = 10.0;

const PLAYER_ACCELERATION: f32 = 2.5;
const PLAYER_ROTATION_SPEED: f32 = 5.0;
const PLAYER_MANEUVER_SPEED: f32 = 1.0;

#[derive(Default)]
pub struct PlayerMovementSystem {
    reader: Option<ReaderId<KeyboardEvent>>,
    rotation_factor: i8,
    acceleration_factor: i8,
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        Read<'a, EventChannel<KeyboardEvent>>,
        Read<'a, DeltaTime>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(
            world
                .fetch_mut::<EventChannel<KeyboardEvent>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (player, mut pos, mut vel, events, dt): Self::SystemData) {
        handle_inputs(
            events,
            &mut self.reader.as_mut().unwrap(),
            &mut self.rotation_factor,
            &mut self.acceleration_factor,
        );

        let delta = dt.0.as_secs_f32();

        for (_, transform, velocity) in (&player, &mut pos, &mut vel).join() {
            if self.acceleration_factor != 0 {
                velocity.speed += PLAYER_ACCELERATION * delta * f32::from(self.acceleration_factor);
                // clamp velocity
                velocity.speed = velocity.speed.clamp(0.0, MAX_PLAYER_SPEED);

                if self.acceleration_factor > 0 {
                    // Make velocity direction vector more like facing direction vector if we're accelerating
                    velocity.direction = velocity
                        .direction
                        .lerp(transform.get_facing_vector(), PLAYER_MANEUVER_SPEED * delta);
                }
            }
            if self.rotation_factor != 0 {
                let (_, _, mut angle) = transform.rotation.to_euler(EulerRot::XYZ);
                angle += PLAYER_ROTATION_SPEED * delta * f32::from(self.rotation_factor);
                transform.rotation = glam::Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
            }
        }
    }
}

fn handle_inputs(
    events: Read<EventChannel<KeyboardEvent>, DefaultProvider>,
    reader: &mut ReaderId<KeyboardEvent>,
    rotation_factor: &mut i8,
    acceleration_factor: &mut i8,
) {
    // Smells like it belongs somewhere else
    for event in events.read(reader) {
        match event {
            &KeyboardEvent::Pressed(VirtualKeyCode::W) => {
                *acceleration_factor = 1;
            }
            &KeyboardEvent::Released(VirtualKeyCode::W) => {
                *acceleration_factor = 0;
            }
            &KeyboardEvent::Pressed(VirtualKeyCode::S) => {
                *acceleration_factor = -2;
            }
            &KeyboardEvent::Released(VirtualKeyCode::S) => {
                *acceleration_factor = 0;
            }
            &KeyboardEvent::Pressed(VirtualKeyCode::D) => {
                *rotation_factor = -1;
            }
            &KeyboardEvent::Released(VirtualKeyCode::D) => {
                *rotation_factor = 0;
            }
            &KeyboardEvent::Pressed(VirtualKeyCode::A) => {
                *rotation_factor = 1;
            }
            &KeyboardEvent::Released(VirtualKeyCode::A) => {
                *rotation_factor = 0;
            }
            _ => (),
        }
    }
}
