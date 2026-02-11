use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct RenameCalendarCommand {
    calendar_id: CalendarId,
    new_name: String,
}

pub struct RenameCalendarHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> RenameCalendarHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RenameCalendarCommand,
    ) -> Result<(), ApplicationError> {
        let mut calendar = self
            .repository
            .find_by_id(&command.calendar_id)
            .await?
            .ok_or(ApplicationError::CalendarNotFound)?;

        calendar.update_name(command.new_name);

        self.repository.save(&calendar).await?;

        Ok(())
    }
}
