#[macro_export]
macro_rules! log_info {
    ($category:expr, $($arg:tt)+) => {{
        $crate::logger::LOGGER.log(
            $crate::logger::LogLevel::Info,
            $category,
            format!($($arg)+),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($category:expr, $($arg:tt)+) => {{
        $crate::logger::LOGGER.log(
            $crate::logger::LogLevel::Warn,
            $category,
            format!($($arg)+),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! log_error {
    ($category:expr, $($arg:tt)+) => {{
        $crate::logger::LOGGER.log(
            $crate::logger::LogLevel::Error,
            $category,
            format!($($arg)+),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($category:expr, $($arg:tt)+) => {{
        $crate::logger::LOGGER.log(
            $crate::logger::LogLevel::Debug,
            $category,
            format!($($arg)+),
            file!(),
            line!(),
        );
    }};
}

#[macro_export]
macro_rules! log_critical {
    ($category:expr, $($arg:tt)+) => {{
        $crate::logger::LOGGER.log(
            $crate::logger::LogLevel::Critical,
            $category,
            format!($($arg)+),
            file!(),
            line!(),
        );
    }};
}
