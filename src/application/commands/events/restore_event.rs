use crate::{
    application::error::ApplicationError,
    domain::{repository::EventRepository, value_objects::EventId}
};

pub struct RestoreEventCommand {
    id: EventId,
}

pub struct RestoreEventHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> RestoreEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RestoreEventCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.restore();

        self.repository.save(&event).await?;

        Ok(())
    }
}
