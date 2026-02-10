use crate::{
    application::error::ApplicationError,
    domain::{repository::EventRepository, value_objects::EventId}
};

pub struct UpdateEventTitleCommand {
    id: EventId,
    new_title: String,
}

pub struct UpdateEventTitleHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> UpdateEventTitleHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateEventTitleCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.update_title(command.new_title);

        self.repository.save(&event).await?;

        Ok(())
    }
}
