
pub mod commands;

pub mod util;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    TestParameterizedError {
        expected: String,
        found: String,
    },
    #[error("failed calling jira")]
    ApiCallFailed(#[from] reqwest::Error),
    #[error("failed calling jira {0}")]
    ApiCallBadStatus(String),
    #[error("failed deserializing response")]
    DeserializationError,
    #[error("Something bad happened and we got no idea what")]
    UnknownShennanigans
}
