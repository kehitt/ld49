use std::time::Duration;

use winit::event::VirtualKeyCode;

#[derive(Default)]
pub struct DeltaTime(pub Duration);

#[derive(Debug)]
pub enum WindowEvent {
    Resize(u32, u32),
}

#[derive(Debug)]
pub enum KeyboardEvent {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}
