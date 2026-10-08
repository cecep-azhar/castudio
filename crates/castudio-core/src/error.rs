use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudioError {
    #[error("CAS-DB-001: Database error: {0}")]
    Db(String),

    #[error("CAS-AI-002: AI Provider error: {0}")]
    Ai(String),

    #[error("CAS-NOTFOUND-001: Resource not found: {0}")]
    NotFound(String),

    #[error("CAS-VAL-001: Validation error: {0}")]
    Validation(String),

    #[error("CAS-EXP-001: Export error: {0}")]
    Export(String),

    #[error("CAS-AUT-001: Automation dispatch error: {0}")]
    Dispatch(String),

    #[error("CAS-IO-001: IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<rusqlite::Error> for StudioError {
    fn from(err: rusqlite::Error) -> Self {
        StudioError::Db(err.to_string())
    }
}

impl Serialize for StudioError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
