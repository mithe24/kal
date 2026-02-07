use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct DeleteCalendarCommand {
    id: CalendarId,
}
pub struct DeleteCalendarHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> DeleteCalendarHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: DeleteCalendarCommand,
    ) -> Result<(), ApplicationError> {
        let _calendar = self
            .repository
            .find_by_id(&command.id)
            .await?;

        self.repository.delete(&command.id).await?;

        Ok(())
    }
}
