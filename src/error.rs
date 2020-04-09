use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SQLError {
    query: String,
    error: String,
    position: i32,
}

#[derive(Debug)]
pub enum Error {
    ExecError(reqwest::Error),
    DeserializeError(serde_json::error::Error),
    FileError(std::io::Error),
    SQLError(SQLError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::ExecError(err) => format!("Error executing query: {}", err),
            Error::DeserializeError(err) => format!("Error deserializing json: {}", err),
            Error::SQLError(err) => format!("Error '{}' with '{}' at position '{}'", err.error, err.query, err.position),
            Error::FileError(err) => format!("Failed to open file: {}", err),
        })
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ExecError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::DeserializeError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::FileError(err)
    }
}