use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct DeleteCalendarCommand {
    calendar_id: CalendarId,
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
            .find_by_id(&command.calendar_id)
            .await?;

        self.repository.delete(&command.calendar_id).await?;

        Ok(())
    }
}
