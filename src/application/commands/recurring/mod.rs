// Series-level commands (affect the entire recurring event)
pub mod create_recurring_event;
pub mod cancel_recurring_event;
pub mod restore_recurring_event;
pub mod delete_recurring_event;

// Occurrence-level commands (affect single instances)
pub mod cancel_recurring_occurrence;
pub mod restore_recurring_occurrence;
pub mod reschedule_recurring_occurrence;

// Re-exports for convenience
pub use create_recurring_event::{CreateRecurringEventCommand, CreateRecurringEventHandler};
pub use cancel_recurring_event::{CancelRecurringEventCommand, CancelRecurringEventHandler};
pub use restore_recurring_event::{RestoreRecurringEventCommand, RestoreRecurringEventHandler};
pub use delete_recurring_event::{DeleteRecurringEventCommand, DeleteRecurringEventHandler};
pub use cancel_recurring_occurrence::{CancelRecurringOccurrenceCommand, CancelRecurringOccurrenceHandler};
pub use restore_recurring_occurrence::{RestoreRecurringOccurrenceCommand, RestoreRecurringOccurrenceHandler};
pub use reschedule_recurring_occurrence::{RescheduleRecurringOccurrenceCommand, RescheduleRecurringOccurrenceHandler};
