use thiserror::Error;
use crate::domain::{
    error::DomainError,
    repository::RepositoryError,
    value_objects::{
        CalendarIdError,
        EventIdError,
        TimeRangeError,
        FrequencyError
    },
};

#[derive(Debug, Error)]
pub enum ApplicationError {

    // Domain-related errors
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),


    // Business rule violations
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),


    // Conflict errors (e.g., duplicate, constraint violations)
    #[error("Conflict: {0}")]
    Conflict(String),


    // Permission/authorization errors
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),


    // Calendar-specific errors
    #[error("Calendar '{0}' is archived")]
    CalendarArchived(String),

    #[error("Calendar '{0}' is not archived")]
    CalendarNotArchived(String),

    #[error("Calendar '{0}' has active events and cannot be deleted")]
    CalendarHasEvents(String),

    #[error("Calendar '{0}' does not exist")]
    CalendarNotFound(String),


    // Event-specific errors
    #[error("Event '{0}' does not exist")]
    EventNotFound(String),

    #[error("Event '{0}' is cancelled")]
    EventCancelled(String),

    #[error("Event '{0}' is not cancelled")]
    EventNotCancelled(String),

    #[error("Event '{0}' does not belong to calendar '{1}'")]
    EventNotInCalendar(String, String),

    #[error("Event time conflicts with existing event")]
    EventConflict,


    // Recurring event errors
    #[error("Recurring event '{0}' does not exist")]
    RecurringEventNotFound(String),

    #[error("Recurring event '{0}' is cancelled")]
    RecurringEventCancelled(String),

    #[error("Recurring event '{0}' is not cancelled")]
    RecurringEventNotCancelled(String),

    #[error("Recurring event '{0}' does not belong to calendar '{1}'")]
    RecurringEventNotInCalendar(String, String),


    // Occurrence-specific errors
    #[error("Occurrence on {0} does not exist for recurring event '{1}'")]
    OccurrenceNotFound(String, String),

    #[error("Occurrence on {0} is already cancelled")]
    OccurrenceAlreadyCancelled(String),

    #[error("Occurrence on {0} is not cancelled")]
    OccurrenceNotCancelled(String),

    #[error("Occurrence on {0} is already rescheduled")]
    OccurrenceAlreadyRescheduled(String),

    #[error("Occurrence on {0} is not rescheduled")]
    OccurrenceNotRescheduled(String),


    // Internal/unexpected errors
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ApplicationError {
    /// Create a NotFound error for a calendar
    pub fn calendar_not_found(calendar_id: &str) -> Self {
        Self::CalendarNotFound(calendar_id.to_string())
    }

    /// Create a NotFound error for an event
    pub fn event_not_found(event_id: &str) -> Self {
        Self::EventNotFound(event_id.to_string())
    }

    /// Create a NotFound error for a recurring event
    pub fn recurring_event_not_found(event_id: &str) -> Self {
        Self::RecurringEventNotFound(event_id.to_string())
    }

    /// Create an error for event not in calendar
    pub fn event_not_in_calendar(event_id: &str, calendar_id: &str) -> Self {
        Self::EventNotInCalendar(event_id.to_string(), calendar_id.to_string())
    }

    /// Create an error for recurring event not in calendar
    pub fn recurring_event_not_in_calendar(
        event_id: &str,
        calendar_id: &str
    ) -> Self {
        Self::RecurringEventNotInCalendar(
            event_id.to_string(),
            calendar_id.to_string(),
        )
    }

    /// Create a Conflict error for duplicate resources
    pub fn duplicate(resource: &str, field: &str, value: &str) -> Self {
        Self::Conflict(format!(
            "{} with {} '{}' already exists",
            resource,
            field,
            value,
        ))
    }

    /// Create an error for archived calendar
    pub fn calendar_archived(calendar_id: &str) -> Self {
        Self::CalendarArchived(calendar_id.to_string())
    }

    /// Create an error for not archived calendar
    pub fn calendar_not_archived(calendar_id: &str) -> Self {
        Self::CalendarNotArchived(calendar_id.to_string())
    }

    /// Create an error for calendar with events
    pub fn calendar_has_events(calendar_id: &str) -> Self {
        Self::CalendarHasEvents(calendar_id.to_string())
    }

    /// Create an error for cancelled event
    pub fn event_cancelled(event_id: &str) -> Self {
        Self::EventCancelled(event_id.to_string())
    }

    /// Create an error for not cancelled event
    pub fn event_not_cancelled(event_id: &str) -> Self {
        Self::EventNotCancelled(event_id.to_string())
    }

    /// Create an error for cancelled recurring event
    pub fn recurring_event_cancelled(event_id: &str) -> Self {
        Self::RecurringEventCancelled(event_id.to_string())
    }

    /// Create an error for not cancelled recurring event
    pub fn recurring_event_not_cancelled(event_id: &str) -> Self {
        Self::RecurringEventNotCancelled(event_id.to_string())
    }

    /// Create an error for occurrence not found
    pub fn occurrence_not_found(date: &str, event_id: &str) -> Self {
        Self::OccurrenceNotFound(date.to_string(), event_id.to_string())
    }

    /// Create an error for already cancelled occurrence
    pub fn occurrence_already_cancelled(date: &str) -> Self {
        Self::OccurrenceAlreadyCancelled(date.to_string())
    }

    /// Create an error for not cancelled occurrence
    pub fn occurrence_not_cancelled(date: &str) -> Self {
        Self::OccurrenceNotCancelled(date.to_string())
    }

    /// Create an error for already rescheduled occurrence
    pub fn occurrence_already_rescheduled(date: &str) -> Self {
        Self::OccurrenceAlreadyRescheduled(date.to_string())
    }

    /// Create an error for not rescheduled occurrence
    pub fn occurrence_not_rescheduled(date: &str) -> Self {
        Self::OccurrenceNotRescheduled(date.to_string())
    }

    /// Check if this is a NotFound error
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            Self::NotFound(_)
                | Self::CalendarNotFound(_)
                | Self::EventNotFound(_)
                | Self::RecurringEventNotFound(_)
        )
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::InvalidInput(_) | Self::Domain(_))
    }

    pub fn is_conflict(&self) -> bool {
        matches!(self, Self::Conflict(_) | Self::EventConflict)
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotFound(_)
            | Self::CalendarNotFound(_)
            | Self::EventNotFound(_)
            | Self::RecurringEventNotFound(_)
            | Self::OccurrenceNotFound(_, _) => 404,

            Self::InvalidInput(_) | Self::InvalidOperation(_) => 400,

            Self::Conflict(_)
            | Self::EventConflict
            | Self::CalendarHasEvents(_)
            | Self::EventNotInCalendar(_, _)
            | Self::RecurringEventNotInCalendar(_, _) => 409,

            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,

            Self::Domain(_) => 422,

            Self::CalendarArchived(_)
            | Self::CalendarNotArchived(_)
            | Self::EventCancelled(_)
            | Self::EventNotCancelled(_)
            | Self::RecurringEventCancelled(_)
            | Self::RecurringEventNotCancelled(_)
            | Self::OccurrenceAlreadyCancelled(_)
            | Self::OccurrenceNotCancelled(_)
            | Self::OccurrenceAlreadyRescheduled(_)
            | Self::OccurrenceNotRescheduled(_) => 400,

            Self::Repository(_) | Self::Internal(_) => 500,
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NotFound(_)
            | Self::CalendarNotFound(_)
            | Self::EventNotFound(_)
            | Self::RecurringEventNotFound(_)
            | Self::OccurrenceNotFound(_, _) => 1,

            Self::InvalidInput(_)
            | Self::InvalidOperation(_)
            | Self::CalendarArchived(_)
            | Self::CalendarNotArchived(_)
            | Self::EventCancelled(_)
            | Self::EventNotCancelled(_)
            | Self::RecurringEventCancelled(_)
            | Self::RecurringEventNotCancelled(_)
            | Self::OccurrenceAlreadyCancelled(_)
            | Self::OccurrenceNotCancelled(_)
            | Self::OccurrenceAlreadyRescheduled(_)
            | Self::OccurrenceNotRescheduled(_) => 2,

            Self::Conflict(_)
            | Self::EventConflict
            | Self::CalendarHasEvents(_)
            | Self::EventNotInCalendar(_, _)
            | Self::RecurringEventNotInCalendar(_, _) => 3,

            Self::Unauthorized(_) | Self::Forbidden(_) => 4,

            Self::Domain(_) | Self::Repository(_) | Self::Internal(_) => 5,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::NotFound(msg)
            | Self::InvalidInput(msg)
            | Self::InvalidOperation(msg)
            | Self::Conflict(msg)
            | Self::Unauthorized(msg)
            | Self::Forbidden(msg) => msg.clone(),

            Self::CalendarNotFound(id) => format!(
                "Calendar '{}' not found",
                id,
            ),
            Self::EventNotFound(id) => format!("Event '{}' not found", id),
            Self::RecurringEventNotFound(id) => format!(
                "Recurring event '{}' not found",
                id,
            ),

            Self::CalendarArchived(id) => format!(
                "Calendar '{}' is archived",
                id,
            ),
            Self::CalendarNotArchived(id) => format!(
                "Calendar '{}' is not archived",
                id,
            ),
            Self::CalendarHasEvents(id) => {
                format!(
                    "Calendar '{}' has active events and cannot be deleted",
                    id,
                )
            }

            Self::EventCancelled(id) => format!("Event '{}' is cancelled", id),
            Self::EventNotCancelled(id) => format!(
                "Event '{}' is not cancelled",
                id,
            ),
            Self::EventNotInCalendar(event_id, calendar_id) => {
                format!(
                    "Event '{}' does not belong to calendar '{}'",
                    event_id,
                    calendar_id,
                )
            }
            Self::EventConflict => {
                "Event time conflicts with existing event".to_string()
            },

            Self::RecurringEventCancelled(id) => format!(
                "Recurring event '{}' is cancelled",
                id,
            ),
            Self::RecurringEventNotCancelled(id) => {
                format!("Recurring event '{}' is not cancelled", id)
            }
            Self::RecurringEventNotInCalendar(event_id, calendar_id) => format!(
                "Recurring event '{}' does not belong to calendar '{}'",
                event_id,
                calendar_id,
            ),

            Self::OccurrenceNotFound(date, event_id) => {
                format!(
                    "Occurrence on {} does not exist for recurring event '{}'",
                    date,
                    event_id,
                )
            }
            Self::OccurrenceAlreadyCancelled(date) => {
                format!("Occurrence on {} is already cancelled", date)
            }
            Self::OccurrenceNotCancelled(date) => {
                format!("Occurrence on {} is not cancelled", date)
            }
            Self::OccurrenceAlreadyRescheduled(date) => {
                format!("Occurrence on {} is already rescheduled", date)
            }
            Self::OccurrenceNotRescheduled(date) => {
                format!("Occurrence on {} is not rescheduled", date)
            }

            Self::Domain(err) => format!("Validation error: {}", err),
            Self::Repository(_) | Self::Internal(_) => {
                "An internal error occurred. Please try again.".to_string()
            }
        }
    }
}

impl From<CalendarIdError> for ApplicationError {
    fn from(err: CalendarIdError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}

impl From<EventIdError> for ApplicationError {
    fn from(err: EventIdError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}

impl From<TimeRangeError> for ApplicationError {
    fn from(err: TimeRangeError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}

impl From<FrequencyError> for ApplicationError {
    fn from(err: FrequencyError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}

impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => Self::NotFound(
                "Resource not found".to_string()
            ),
            RepositoryError::DatabaseError(msg) => Self::Internal(
                format!("Database error: {}", msg)
            ),
            RepositoryError::MappingError(msg) => Self::Internal(
                format!("Mapping error: {}", msg)
            ),
            RepositoryError::TransactionError(msg) => {
                Self::Internal(format!("Transaction error: {}", msg))
            }
            RepositoryError::ConstraintViolation(msg) => Self::Conflict(msg),
            RepositoryError::ConnectionError(msg) => {
                Self::Internal(format!("Connection error: {}", msg))
            }
        }
    }
}
