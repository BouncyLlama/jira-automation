pub mod commands;

pub mod util;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {

    #[error("failed calling jira")]
    ApiCallFailed(#[from] reqwest::Error),
    #[error("failed calling jira {0}")]
    ApiCallBadStatus(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("failed deserializing response")]
    DeserializationError,

    #[error("query was meant to match exactly one release but found multiple")]
    MatchedMultipleReleases,
    #[error("could not create the requested release")]
    CouldNotCreateRelease,
    #[error("the specified issue transition is unknown")]
    UnknownTransition,
    #[error("no issues were found to release")]
    NoIssuesFound,
}
