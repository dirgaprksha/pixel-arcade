use super::{KeyCode, MouseButton};
use std::collections::HashSet;

pub struct Input {
    pressed_keys: HashSet<KeyCode>,
    pressed_buttons: HashSet<MouseButton>,
    mouse_position: (f32, f32),
}

impl Input {
    // Creates a new input state tracker
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0),
        }
    }

    // Checks if a keyboard key is currently pressed
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    // Internal: Updates key pressed state
    pub(crate) fn set_key_pressed(&mut self, key: KeyCode, is_pressed: bool) {
        if is_pressed {
            self.pressed_keys.insert(key);
        } else {
            self.pressed_keys.remove(&key);
        }
    }

    // hecks if a mouse button is currently pressed
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    // Internal: Updates mouse button pressed state
    pub(crate) fn set_mouse_button_pressed(&mut self, button: MouseButton, is_pressed: bool) {
        if is_pressed {
            self.pressed_buttons.insert(button);
        } else {
            self.pressed_buttons.remove(&button);
        }
    }

    // Returns current mouse cursor position as (x, y)
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    // Internal: Updates mouse position
    pub(crate) fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
}
