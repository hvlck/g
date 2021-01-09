// std
use std::fmt;

// crates

// local

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other(msg) => write!(f, "{}", msg),
            _ => unimplemented!(),
        }
    }
}

impl std::error::Error for Error {}
