use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Hypr(hyprland::error::HyprError),
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Hypr(e) => write!(f, "Hyprland error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl From<hyprland::error::HyprError> for AppError {
    fn from(err: hyprland::error::HyprError) -> Self {
        AppError::Hypr(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<std::ffi::FromBytesUntilNulError> for AppError {
    fn from(err: std::ffi::FromBytesUntilNulError) -> Self {
        AppError::Parse(format!("Failed to parse output: {}", err))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(format!("Failed to parse integer: {}", err))
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        AppError::Parse(format!("Failed to parse UTF-8: {}", err))
    }
}
