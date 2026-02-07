use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct UnarchiveCalendarCommand {
    id: CalendarId,
}

pub struct UnarchiveCalendarHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> UnarchiveCalendarHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UnarchiveCalendarCommand,
    ) -> Result<(), ApplicationError> {
        let mut calendar = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::CalendarNotFound)?;

        calendar.unarchive();

        self.repository.save(&calendar).await?;

        Ok(())
    }
}
