pub struct ImageData {
    pub rgba_bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    // Creates an ImageData from decoded RGBA bytes
    pub fn from_bytes(rgba_bytes: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            rgba_bytes,
            width,
            height,
        }
    }
}
