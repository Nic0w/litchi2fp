use std::fmt::Display;

use serde::de;

#[derive(Debug)]
pub enum Error {
    UnsupportedValue,
    BadMagic,
    Custom(String)
    
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedValue => write!(f, "unsupported value"),
            Self::BadMagic => write!(f, "file does not begin with bytes 'lchm'"),
            Self::Custom(text) => write!(f, "{}", text),
        }
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Error::Custom(msg.to_string())
    }
}