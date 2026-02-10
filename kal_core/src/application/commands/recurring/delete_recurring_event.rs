use crate::{
    application::error::ApplicationError,
    domain::{repository::RecurringEventRepository, value_objects::EventId}
};

pub struct DeleteRecurringEventCommand {
    pub id: EventId,
}

pub struct DeleteRecurringEventHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> DeleteRecurringEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: DeleteRecurringEventCommand
    ) -> Result<(), ApplicationError> {

        let _event = self
            .repository
            .find_by_id(&command.id)
            .await?;

        self.repository.delete(&command.id).await?;

        Ok(())
    }
}
