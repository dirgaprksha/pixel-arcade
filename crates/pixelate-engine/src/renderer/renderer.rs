use super::RendererError;
use crate::{assets::ImageData, log_debug, log_info, window::Window};
use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use winit::window::Window as WinitWindow;

pub struct Renderer {
    surface: Surface<Arc<WinitWindow>, Arc<WinitWindow>>,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl Renderer {
    // Creates a new renderer for given window
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let inner_window = window.inner();

        let context = Context::new(inner_window.clone())
            .map_err(|source| RendererError::ContextCreationFailed { source })?;

        let surface = Surface::new(&context, inner_window)
            .map_err(|source| RendererError::SurfaceCreationFailed { source })?;

        let (width, height) = window.inner_size();

        log_info!("Renderer", "Created renderer ({}x{})", width, height);

        Ok(Self {
            surface,
            buffer: vec![0; (width * height) as usize],
            width,
            height,
        })
    }

    // Resizes render buffer
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), RendererError> {
        if self.width == width && self.height == height {
            return Ok(());
        }

        log_debug!(
            "Renderer",
            "Resizing from {}x{} to {}x{}",
            self.width,
            self.height,
            width,
            height
        );

        self.width = width;
        self.height = height;
        self.buffer.resize((width * height) as usize, 0);

        if let (Some(width), Some(height)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface
                .resize(width, height)
                .map_err(|source| RendererError::ResizeFailed {
                    width: width.get(),
                    height: height.get(),
                    source,
                })?;
        }

        Ok(())
    }

    // Clears buffer with a solid color
    pub fn clear(&mut self, color: [u8; 4]) {
        let color_u32 = Self::rgba_to_u32(color);

        self.buffer.fill(color_u32);
    }

    // Draws an image at specified position with scaling
    pub fn draw_image(&mut self, image: &ImageData, x: i32, y: i32, size: u32) {
        let image_rgba_bytes = &image.rgba_bytes;
        let image_width = image.width;
        let image_height = image.height;

        for destination_y in 0..size {
            for destination_x in 0..size {
                let screen_x = x + destination_x as i32;
                let screen_y = y + destination_y as i32;

                if screen_x < 0
                    || screen_y < 0
                    || screen_x >= self.width as i32
                    || screen_y >= self.height as i32
                {
                    continue;
                }

                let source_x = (destination_x * image_width) / size;
                let source_y = (destination_y * image_height) / size;
                let source_index = ((source_y * image_width + source_x) * 4) as usize;

                let alpha = image_rgba_bytes[source_index + 3];
                if alpha == 0 {
                    continue;
                }

                let rgba = [
                    image_rgba_bytes[source_index],
                    image_rgba_bytes[source_index + 1],
                    image_rgba_bytes[source_index + 2],
                    alpha,
                ];

                let buffer_index = (screen_y as u32 * self.width + screen_x as u32) as usize;

                self.buffer[buffer_index] = Self::rgba_to_u32(rgba);
            }
        }
    }

    // Draws a filled rectangle
    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) {
        let color_u32 = Self::rgba_to_u32(color);

        for offset_y in 0..height {
            for offset_x in 0..width {
                let screen_x = x + offset_x as i32;
                let screen_y = y + offset_y as i32;

                if screen_x < 0
                    || screen_y < 0
                    || screen_x >= self.width as i32
                    || screen_y >= self.height as i32
                {
                    continue;
                }

                let buffer_index = (screen_y as u32 * self.width + screen_x as u32) as usize;

                self.buffer[buffer_index] = color_u32;
            }
        }
    }

    // Draws a line between two points
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: [u8; 4]) {
        let delta_x = (x2 - x1).abs();
        let delta_y = -(y2 - y1).abs();
        let step_x = if x1 < x2 { 1 } else { -1 };
        let step_y = if y1 < y2 { 1 } else { -1 };
        let mut error = delta_x + delta_y;

        let mut current_x = x1;
        let mut current_y = y1;

        loop {
            self.draw_point(current_x, current_y, color);

            if current_x == x2 && current_y == y2 {
                break;
            }

            let error_doubled = 2 * error;
            if error_doubled >= delta_y {
                if current_x == x2 {
                    break;
                }

                error += delta_y;
                current_x += step_x;
            }

            if error_doubled <= delta_x {
                if current_y == y2 {
                    break;
                }

                error += delta_x;
                current_y += step_y;
            }
        }
    }

    // Draws a circle outline using midpoint circle
    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: u32, color: [u8; 4]) {
        if radius == 0 {
            self.draw_point(center_x, center_y, color);
            return;
        }

        let mut offset_x = 0i32;
        let mut offset_y = radius as i32;
        let mut decision_parameter = 1 - radius as i32;

        while offset_x <= offset_y {
            self.draw_point(center_x + offset_x, center_y + offset_y, color);
            self.draw_point(center_x - offset_x, center_y + offset_y, color);
            self.draw_point(center_x + offset_x, center_y - offset_y, color);
            self.draw_point(center_x - offset_x, center_y - offset_y, color);
            self.draw_point(center_x + offset_y, center_y + offset_x, color);
            self.draw_point(center_x - offset_y, center_y + offset_x, color);
            self.draw_point(center_x + offset_y, center_y - offset_x, color);
            self.draw_point(center_x - offset_y, center_y - offset_x, color);

            offset_x += 1;

            if decision_parameter < 0 {
                decision_parameter += 2 * offset_x + 1;
            } else {
                offset_y -= 1;
                decision_parameter += 2 * (offset_x - offset_y) + 1;
            }
        }
    }

    // Draws a single pixel at specified coordinates
    pub fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4]) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        let buffer_index = (y as u32 * self.width + x as u32) as usize;

        self.buffer[buffer_index] = Self::rgba_to_u32(color);
    }

    // Presents rendered buffer to window
    pub fn present(&mut self) -> Result<(), RendererError> {
        let mut surface_buffer = self
            .surface
            .buffer_mut()
            .map_err(|source| RendererError::PresentFailed { source })?;

        surface_buffer.copy_from_slice(&self.buffer);

        surface_buffer
            .present()
            .map_err(|source| RendererError::PresentFailed { source })?;

        Ok(())
    }

    // Converts RGBA color to u32 format for buffer
    fn rgba_to_u32(rgba: [u8; 4]) -> u32 {
        let [r, g, b, _a] = rgba;

        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}
