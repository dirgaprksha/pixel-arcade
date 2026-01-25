use crate::{
    input::{Event, Input},
    renderer::Renderer,
    window::Window,
};
use std::error::Error;

pub trait Application {
    // Called once when application starts
    fn on_init(&mut self, window: &Window) -> Result<(), Box<dyn Error>> {
        let _ = window;

        Ok(())
    }

    // Called every frame to update game logic
    fn on_update(&mut self, window: &Window, input: &Input, delta_time: f32) {
        let _ = (window, input, delta_time);
    }

    // Called when discrete input or window events occur
    fn on_event(&mut self, window: &Window, event: Event) {
        let _ = (window, event);
    }

    // Called every frame to render graphics
    fn on_render(&mut self, window: &Window, renderer: &mut Renderer) {
        let _ = (window, renderer);
    }

    // Called once before application exits
    fn on_shutdown(&mut self, window: &Window) {
        let _ = window;
    }
}
