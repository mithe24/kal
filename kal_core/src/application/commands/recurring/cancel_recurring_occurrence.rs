use chrono::{DateTime, Utc};

use crate::{
    application::error::ApplicationError,
    domain::{repository::RecurringEventRepository, value_objects::EventId}
};

pub struct CancelRecurringOccurrenceCommand {
    id: EventId,
    starts_at: DateTime<Utc>,
}

pub struct CancelRecurringOccurrenceHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> CancelRecurringOccurrenceHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CancelRecurringOccurrenceCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        event.cancel_occurrence(command.starts_at);

        self.repository.save(&event).await?;

        Ok(())
    }
}
