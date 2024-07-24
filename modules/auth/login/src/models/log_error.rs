use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LogError {
    NotFound,
    PasswordError,
}

impl Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LogError::NotFound => write!(f, "LogError: NotFound"),
            LogError::PasswordError => write!(f, "LogError: PasswordError"),
        }
    }
}

impl Error for LogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
