use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum MapError {
    InvalidDigit(char),
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapError::InvalidDigit(ch) => {
                write!(f, "Invalid digit '{}': only digits 0-9 are allowed", ch)
            }
        }
    }
}

impl Error for MapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MapError::InvalidDigit(_) => None,
        }
    }
}
