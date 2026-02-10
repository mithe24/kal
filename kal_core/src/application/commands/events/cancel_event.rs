use crate::{
    application::error::ApplicationError,
    domain::{repository::EventRepository, value_objects::EventId}
};

pub struct CancelEventCommand {
    id: EventId,
}

pub struct CancelEventHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> CancelEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CancelEventCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.cancel();

        self.repository.save(&event).await?;

        Ok(())
    }
}
