pub struct WindowConfiguration {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowConfiguration {
    // Creates a new window configuration with specified parameters
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 800,
        }
    }
}
