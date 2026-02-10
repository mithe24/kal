use crate::{
    application::error::ApplicationError,
    domain::{repository::EventRepository, value_objects::EventId}
};

pub struct UpdateEventDescriptionCommand {
    id: EventId,
    new_description: Option<String>,
}

pub struct UpdateEventDescriptionHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> UpdateEventDescriptionHandler<R> {
    pub fn new(repository: R) ->  Self {
        Self { repository }
    }
    
    pub async fn handle(
        &self,
        command: UpdateEventDescriptionCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.update_description(command.new_description);

        self.repository.save(&event).await?;

        Ok(())
    }
}
