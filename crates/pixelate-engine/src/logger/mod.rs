mod logger;
mod macros;

pub use logger::LogLevel;

pub static LOGGER: logger::Logger = logger::Logger::new(LogLevel::Info);
