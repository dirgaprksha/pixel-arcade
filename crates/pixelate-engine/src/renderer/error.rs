use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum RendererError {
    ContextCreationFailed {
        source: softbuffer::SoftBufferError,
    },
    SurfaceCreationFailed {
        source: softbuffer::SoftBufferError,
    },
    ResizeFailed {
        width: u32,
        height: u32,
        source: softbuffer::SoftBufferError,
    },
    PresentFailed {
        source: softbuffer::SoftBufferError,
    },
}

impl Display for RendererError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            RendererError::ContextCreationFailed { source } => {
                write!(formatter, "Failed to create rendering context: {}", source)
            }
            RendererError::SurfaceCreationFailed { source } => {
                write!(formatter, "Failed to create rendering surface: {}", source)
            }
            RendererError::ResizeFailed {
                width,
                height,
                source,
            } => {
                write!(
                    formatter,
                    "Failed to resize surface to {}x{}: {}",
                    width, height, source
                )
            }
            RendererError::PresentFailed { source } => {
                write!(formatter, "Failed to present rendered frame: {}", source)
            }
        }
    }
}

impl Error for RendererError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RendererError::ContextCreationFailed { source } => Some(source),
            RendererError::SurfaceCreationFailed { source } => Some(source),
            RendererError::ResizeFailed { source, .. } => Some(source),
            RendererError::PresentFailed { source } => Some(source),
        }
    }
}
