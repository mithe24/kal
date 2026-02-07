use chrono::{DateTime, Utc};

use crate::{
    application::error::ApplicationError,
    domain::{repository::RecurringEventRepository, value_objects::EventId}
};

pub struct RestoreRecurringOccurrenceCommand {
    id: EventId,
    starts_at: DateTime<Utc>,
}

pub struct RestoreRecurringOccurrenceHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> RestoreRecurringOccurrenceHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RestoreRecurringOccurrenceCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        event.restore_occurrence(command.starts_at);

        self.repository.save(&event).await?;

        Ok(())
    }
}
