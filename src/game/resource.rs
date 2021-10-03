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

#[derive(Debug)]
pub enum GameState {
    GameStateInit {},
    GameStatePlay {},
    GameStateEnd {},
}

impl Default for GameState {
    fn default() -> Self {
        GameState::GameStateInit {}
    }
}

#[derive(Debug, Default)]
pub struct GameWindowSize(pub u32, pub u32);
