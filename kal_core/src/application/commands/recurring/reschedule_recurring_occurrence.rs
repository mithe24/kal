use chrono::{DateTime, Utc};

use crate::{application::error::ApplicationError, domain::{
    repository::RecurringEventRepository,
    value_objects::{EventId, TimeRange}}};

pub struct RescheduleRecurringOccurrenceCommand {
    id: EventId,
    orignal_starts_at: DateTime<Utc>,
    new_time_range: TimeRange,
}

pub struct RescheduleRecurringOccurrenceHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> RescheduleRecurringOccurrenceHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RescheduleRecurringOccurrenceCommand
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        event.reschedule_occurrence(
            command.orignal_starts_at,
            command.new_time_range,
        );

        self.repository.save(&event).await?;

        Ok(())
    }
}
