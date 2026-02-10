use crate::{
    application::error::ApplicationError,
    domain::{
        repository::EventRepository,
        value_objects::{EventColor, EventId}
    }
};

pub struct UpdateEventColorCommand {
    id: EventId,
    new_color: EventColor,
}

pub struct UpdateEventColorHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> UpdateEventColorHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateEventColorCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.update_color(command.new_color);

        self.repository.save(&event).await?;

        Ok(())
    }
}
