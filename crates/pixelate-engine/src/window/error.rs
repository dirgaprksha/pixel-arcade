use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};
use winit::error::OsError;

#[derive(Debug)]
pub enum WindowError {
    CreationFailed {
        source: OsError,
        config_summary: String,
    },
}

impl Display for WindowError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            WindowError::CreationFailed {
                source,
                config_summary,
            } => {
                write!(
                    formatter,
                    "Failed to create window ({}): {}",
                    config_summary, source
                )
            }
        }
    }
}

impl Error for WindowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WindowError::CreationFailed { source, .. } => Some(source),
        }
    }
}
