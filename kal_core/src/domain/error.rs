use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    // Calendar errors
    #[error("Calendar name cannot be empty")]
    EmptyCalendarName,

    #[error("Calendar name is too long (max 255 characters)")]
    CalendarNameTooLong,

    #[error("Calendar is archived and cannot be modified")]
    CalendarArchived,

    #[error("Calendar is not archived")]
    CalendarNotArchived,

    // Event errors
    #[error("Event title cannot be empty")]
    EmptyEventTitle,

    #[error("Event title is too long (max 255 characters)")]
    EventTitleTooLong,

    #[error("Event is already cancelled")]
    EventAlreadyCancelled,

    #[error("Event is not cancelled")]
    EventNotCancelled,

    #[error("Cannot modify a cancelled event")]
    CannotModifyCancelledEvent,

    // Time range errors
    #[error("End time must be after start time")]
    InvalidTimeRange,

    #[error("Time range is too long (max 365 days)")]
    TimeRangeTooLong,

    // Recurrence errors
    #[error("Recurrence interval must be at least 1")]
    InvalidRecurrenceInterval,

    #[error("Recurrence end date must be after start date")]
    InvalidRecurrenceEndDate,

    #[error("Recurring event is already cancelled")]
    RecurringEventAlreadyCancelled,

    #[error("Recurring event is not cancelled")]
    RecurringEventNotCancelled,

    #[error("Cannot modify a cancelled recurring event")]
    CannotModifyCancelledRecurringEvent,

    // Occurrence errors
    #[error("Occurrence on {0} does not exist")]
    OccurrenceNotFound(String),

    #[error("Occurrence on {0} is already cancelled")]
    OccurrenceAlreadyCancelled(String),

    #[error("Occurrence on {0} is not cancelled")]
    OccurrenceNotCancelled(String),

    #[error("Occurrence on {0} is already rescheduled")]
    OccurrenceAlreadyRescheduled(String),

    #[error("Occurrence on {0} is not rescheduled")]
    OccurrenceNotRescheduled(String),

    #[error("Cannot modify occurrence on {0}")]
    CannotModifyOccurrence(String),

    // Value object errors
    #[error("Invalid calendar ID: {0}")]
    InvalidCalendarId(String),

    #[error("Invalid event ID: {0}")]
    InvalidEventId(String),

    #[error("Invalid color value: {0}")]
    InvalidColor(u8),

    #[error("Invalid frequency: {0}")]
    InvalidFrequency(String),
}
