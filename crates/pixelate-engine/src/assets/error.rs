use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    io::Error as IOError,
};

#[derive(Debug)]
pub enum AssetError {
    FileRead {
        path: String,
        source: IOError,
    },
    ImageDecode {
        path: String,
        source: image::ImageError,
    },
    NotLoaded {
        path: String,
    },
    InvalidDimensions {
        path: String,
        expected_size: usize,
        actual_size: usize,
    },
}

impl Display for AssetError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AssetError::FileRead { path, source } => {
                write!(formatter, "Failed to read file '{}': {}", path, source)
            }
            AssetError::ImageDecode { path, source } => {
                write!(formatter, "Failed to decode image '{}': {}", path, source)
            }
            AssetError::NotLoaded { path } => {
                write!(formatter, "Asset '{}' is not loaded", path)
            }
            AssetError::InvalidDimensions {
                path,
                expected_size,
                actual_size,
            } => {
                write!(
                    formatter,
                    "Invalid image data for '{}': expected {} bytes, got {} bytes",
                    path, expected_size, actual_size
                )
            }
        }
    }
}

impl Error for AssetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AssetError::FileRead { source, .. } => Some(source),
            AssetError::ImageDecode { source, .. } => Some(source),
            _ => None,
        }
    }
}
