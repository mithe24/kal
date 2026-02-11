use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error)]
pub enum MapperError {
    #[error("Invalid ID: {0}")]
    InvalidId(String),

    #[error("Invalid datetime: {0}")]
    InvalidDate(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}
