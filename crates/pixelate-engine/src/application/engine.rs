use crate::{
    application::Application,
    input::{Event, Input, KeyCode, MouseButton},
    log_error,
    renderer::Renderer,
    window::{Window, WindowConfiguration},
};
use std::{error::Error, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::WindowId,
};

pub struct Engine<A: Application> {
    app: A,
    window: Option<Window>,
    window_config: WindowConfiguration,
    renderer: Option<Renderer>,
    input: Input,
    last_frame: Instant,
}

impl<A: Application> Engine<A> {
    // Creates a new engine instance
    fn new(app: A, window_config: WindowConfiguration) -> Self {
        Self {
            app,
            window: None,
            window_config,
            renderer: None,
            input: Input::new(),
            last_frame: Instant::now(),
        }
    }

    // Runs application with given configuration
    pub fn run(app: A, window_config: WindowConfiguration) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut engine = Self::new(app, window_config);
        event_loop.run_app(&mut engine)?;

        Ok(())
    }
}

impl<A: Application> ApplicationHandler for Engine<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            match Window::new(event_loop, &self.window_config) {
                Ok(window) => match Renderer::new(&window) {
                    Ok(renderer) => {
                        if let Err(error) = self.app.on_init(&window) {
                            log_error!("Engine", "Failed to initialize application: {}", error);
                            event_loop.exit();
                            return;
                        }

                        self.renderer = Some(renderer);
                        self.window = Some(window);
                    }
                    Err(error) => {
                        log_error!("Engine", "Failed to create renderer: {}", error);
                        event_loop.exit();
                    }
                },
                Err(error) => {
                    log_error!("Engine", "Failed to create window: {}", error);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                self.app.on_shutdown(window);

                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    let key = KeyCode::from(key_code);
                    let pressed = event.state == ElementState::Pressed;
                    self.input.set_key_pressed(key, pressed);

                    let input_event = match event.state {
                        ElementState::Pressed => Event::KeyPressed(key),
                        ElementState::Released => Event::KeyReleased(key),
                    };

                    self.app.on_event(window, input_event);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = MouseButton::from(button);
                let pressed = state == ElementState::Pressed;
                self.input.set_mouse_button_pressed(button, pressed);

                let input_event = match state {
                    ElementState::Pressed => Event::MousePressed(button),
                    ElementState::Released => Event::MouseReleased(button),
                };

                self.app.on_event(window, input_event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input
                    .set_mouse_position(position.x as f32, position.y as f32);

                let input_event = Event::MouseMoved {
                    x: position.x as f32,
                    y: position.y as f32,
                };

                self.app.on_event(window, input_event);
            }
            WindowEvent::Resized(size) => {
                if let Err(error) = renderer.resize(size.width, size.height) {
                    log_error!("Engine", "Failed to resize renderer: {}", error);
                }

                let input_event = Event::WindowResized {
                    width: size.width,
                    height: size.height,
                };

                self.app.on_event(window, input_event);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                self.app.on_update(window, &self.input, delta_time);

                self.app.on_render(window, renderer);

                if let Err(error) = renderer.present() {
                    log_error!("Engine", "Failed to present frame: {}", error);
                }

                window.request_redraw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
