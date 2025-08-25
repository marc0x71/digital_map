use std::error::Error;
use std::fmt;

/// Errori che possono verificarsi durante il parsing
#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerError {
    InvalidDigit(char),
    MissingClosingBracket,
    UnexpectedChar(char),
    UnexpectedEmptyRange,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::InvalidDigit(ch) => {
                write!(
                    f,
                    "Invalid digit '{}': only digits 0-9 are allowed inside brackets",
                    ch
                )
            }
            TokenizerError::MissingClosingBracket => {
                write!(
                    f,
                    "Missing closing bracket ']': every '[' must have a corresponding ']'"
                )
            }
            TokenizerError::UnexpectedChar(ch) => {
                write!(
                    f,
                    "Unexpected character '{}': only digits, '[', ']', '*', and '+' are allowed",
                    ch
                )
            }
            TokenizerError::UnexpectedEmptyRange => {
                write!(
                    f,
                    "Empty bracket range '[]': brackets must contain at least one digit"
                )
            }
        }
    }
}

impl Error for TokenizerError {}

#[derive(Debug, Clone, PartialEq)]
pub enum MapError {
    InvalidDigit(char),
    PathAlreadyExists,
    TokenizeError(TokenizerError),
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapError::InvalidDigit(ch) => {
                write!(
                    f,
                    "Invalid digit '{}': input must contain only numeric characters (0-9)",
                    ch
                )
            }
            MapError::TokenizeError(tokenizer_err) => {
                write!(f, "Pattern parsing failed: {}", tokenizer_err)
            }
            MapError::PathAlreadyExists => write!(f, "Path already exists"),
        }
    }
}

// Conversione automatica da TokenizerError a MapError
impl From<TokenizerError> for MapError {
    fn from(error: TokenizerError) -> Self {
        MapError::TokenizeError(error)
    }
}

impl Error for MapError {}
