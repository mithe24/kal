use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct UpdateCalendarDescriptionCommand {
    id: CalendarId,
    new_description: Option<String>,
}

pub struct UpdateCalendarDescriptionHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> UpdateCalendarDescriptionHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateCalendarDescriptionCommand,
    ) -> Result<(), ApplicationError> {
        let mut calendar = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::CalendarNotFound)?;

        calendar.update_description(command.new_description);

        self.repository.save(&calendar).await?;

        Ok(())
    }
}
