use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Invalid time range: start time must be before end time")]
    InvalidTimeRange,

    #[error("Invalid color value")]
    InvalidColor,

    #[error("Invalid frequency")]
    InvalidFrequency,

    #[error("Invalid interval: must be greater than 0")]
    InvalidInterval,

    #[error("Calendar not found: {0}")]
    CalendarNotFound(String),

    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Recurrence not found: {0}")]
    RecurrenceNotFound(String),

    #[error("Cannot modify archived calendar")]
    CalendarArchived,
}
