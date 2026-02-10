pub mod calendar;
pub mod event;
pub mod recurrence;
pub mod value_objects;
pub mod repository;
pub mod error;

// Re-export
pub use calendar::Calendar;
pub use event::Event;
pub use recurrence::{RecurringEvent, RecurrenceRule, RecurrenceException, ExceptionModification};
pub use value_objects::{CalendarId, EventId, TimeRange, Frequency, EventColor};
