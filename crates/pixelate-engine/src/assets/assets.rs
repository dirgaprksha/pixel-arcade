use crate::{AssetError, ImageData};
use image::{load_from_memory, GenericImageView};
use std::{collections::HashMap, fs::read};

pub struct AssetManager {
    image_cache: HashMap<String, ImageData>,
}

impl AssetManager {
    // Creates a new empty asset manager
    pub fn new() -> Self {
        Self {
            image_cache: HashMap::new(),
        }
    }

    // Loads an image from specified path
    pub fn load_image(&mut self, path: &str) -> Result<bool, AssetError> {
        if self.image_cache.contains_key(path) {
            return Ok(false);
        }

        let bytes = read(path).map_err(|error| AssetError::FileRead {
            path: path.to_string(),
            source: error,
        })?;

        let decoded_image = load_from_memory(&bytes).map_err(|error| AssetError::ImageDecode {
            path: path.to_string(),
            source: error,
        })?;

        let (width, height) = decoded_image.dimensions();
        let rgba_bytes = decoded_image.into_rgba8().into_raw();

        self.image_cache.insert(
            path.to_string(),
            ImageData::from_bytes(rgba_bytes, width, height),
        );

        Ok(true)
    }

    // Checks if an image at given path is currently loaded
    pub fn is_loaded(&self, path: &str) -> bool {
        self.image_cache.contains_key(path)
    }

    // Retrieves a reference to a loaded image
    pub fn image(&self, path: &str) -> Result<&ImageData, AssetError> {
        self.image_cache
            .get(path)
            .ok_or_else(|| AssetError::NotLoaded {
                path: path.to_string(),
            })
    }

    // Unloads an image from memory
    pub fn unload_image(&mut self, path: &str) -> bool {
        self.image_cache.remove(path).is_some()
    }

    // Clears all loaded images from memory
    pub fn clear_all(&mut self) {
        self.image_cache.clear();
    }
}
