use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("invalid field '{field}': {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error("missing required field: {0}")]
    MissingField(&'static str),

    #[error("invalid state transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("unsupported message version: {0}")]
    UnsupportedVersion(String),

    #[error("protocol error {code}: {description}")]
    Protocol { code: String, description: String },
}

pub type Result<T> = std::result::Result<T, Error>;
