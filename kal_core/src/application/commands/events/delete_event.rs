use crate::{
    application::error::ApplicationError,
    domain::{repository::EventRepository, value_objects::EventId}
};

pub struct DeleteEventCommand {
    id: EventId,
}

pub struct DeleteEventHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> DeleteEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: DeleteEventCommand,
    ) -> Result<(), ApplicationError> {
        let _event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        self.repository.delete(&command.id).await?;

        Ok(())
    }
}
