use super::{configuration::WindowConfiguration, error::WindowError};
use crate::log_info;
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowAttributes},
};

pub struct Window {
    inner_window: Arc<WinitWindow>,
}

impl Window {
    // Creates a new window from given configuration
    pub fn new(
        event_loop: &ActiveEventLoop,
        config: &WindowConfiguration,
    ) -> Result<Self, WindowError> {
        let logical_size = LogicalSize::new(config.width, config.height);
        let attributes = WindowAttributes::default()
            .with_title(config.title.to_string())
            .with_inner_size(logical_size);

        let window =
            event_loop
                .create_window(attributes)
                .map_err(|error| WindowError::CreationFailed {
                    source: error,
                    config_summary: format!(
                        "{}x{} '{}'",
                        config.width, config.height, config.title
                    ),
                })?;

        log_info!(
            "Window",
            "Created window '{}' ({}x{})",
            config.title,
            config.width,
            config.height
        );

        Ok(Self {
            inner_window: Arc::new(window),
        })
    }

    // Returns a cloned Arc reference to underlying winit window
    pub fn inner(&self) -> Arc<WinitWindow> {
        Arc::clone(&self.inner_window)
    }

    // Returns current window title
    pub fn title(&self) -> String {
        self.inner_window.title()
    }

    // Sets window title
    pub fn set_title(&self, title: &str) {
        self.inner_window.set_title(title);
    }

    // Returns inner size of window
    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.inner_window.inner_size();

        (size.width, size.height)
    }

    // Requests a redraw of window
    pub fn request_redraw(&self) {
        self.inner_window.request_redraw();
    }
}
