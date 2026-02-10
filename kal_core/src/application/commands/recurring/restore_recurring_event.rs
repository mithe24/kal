use crate::{
    application::error::ApplicationError,
    domain::{ repository::RecurringEventRepository, value_objects::EventId },
};

pub struct RestoreRecurringEventCommand {
    id: EventId,
}

pub struct RestoreRecurringEventHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> RestoreRecurringEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RestoreRecurringEventCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        event.restore();

        self.repository.save(&event).await?;

        Ok(())
    }
}
