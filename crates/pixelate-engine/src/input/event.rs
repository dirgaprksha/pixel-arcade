use super::{KeyCode, MouseButton};

pub enum Event {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),
    MouseMoved { x: f32, y: f32 },
    WindowResized { width: u32, height: u32 },
}
