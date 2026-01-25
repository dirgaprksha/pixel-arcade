use winit::event::MouseButton as WinitMouseButton;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Right => MouseButton::Right,
            _ => MouseButton::Unknown,
        }
    }
}
