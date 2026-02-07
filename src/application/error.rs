use thiserror::Error;

use crate::domain::{error::DomainError, repository::RepositoryError};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Recurring event not found")]
    RecurringEventNotFound,

    #[error("Calendar not found")]
    CalendarNotFound,

    #[error("Event not found")]
    EventNotFound,

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<RepositoryError> for ApplicationError {
    fn from(error: RepositoryError) -> Self {
        ApplicationError::Repository(error.to_string())
    }
}
