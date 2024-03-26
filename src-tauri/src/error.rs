// use crate::logs;
use serde::{ser::SerializeMap, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidKeeper,
    InvalidReader,
    InvalidParameter,
    UsernameTaken,
    Unexpected,
}

impl Error {
    fn message(&self) -> &'static str {
        match self {
            Self::InvalidKeeper => "invalid keeper",
            Self::InvalidReader => "invalid reader",
            Self::InvalidParameter => "invalid parameter",
            Self::UsernameTaken => "username already registered",
            Self::Unexpected => "unexpected error occurred",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Self::InvalidKeeper => "invalid_keeper",
            Self::InvalidReader => "invalid_reader",
            Self::InvalidParameter => "invalid_parameter",
            Self::UsernameTaken => "username_taken",
            Self::Unexpected => "unexpected",
        }
    }
}

impl<T: std::error::Error> From<T> for Error {
    fn from(e: T) -> Self {
        // logs::error!("Unexpected error", e);
        Self::Unexpected
    }
}

impl Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut json_err = serializer.serialize_map(Some(2))?;
        json_err.serialize_entry("key", self.key())?;
        json_err.serialize_entry("error", self.message())?;
        json_err.end()
    }
}
