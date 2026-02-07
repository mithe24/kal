use chrono::{DateTime, Utc};

use crate::{
    application::error::ApplicationError,
    domain::{ repository::RecurringEventRepository, value_objects::EventId },
};

pub struct CancelRecurringEventCommand {
    id: EventId,
    starts_at: DateTime<Utc>
}

pub struct CancelRecurringEventHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> CancelRecurringEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CancelRecurringEventCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        event.cancel();

        self.repository.save(&event).await?;

        Ok(())
    }
}
