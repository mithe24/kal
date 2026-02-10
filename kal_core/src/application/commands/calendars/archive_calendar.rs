use crate::{
    application::error::ApplicationError,
    domain::{repository::CalendarRepository, value_objects::CalendarId}
};

pub struct ArchiveCalendarCommand {
    id: CalendarId,
}

pub struct ArchiveCalendarHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> ArchiveCalendarHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: ArchiveCalendarCommand,
    ) -> Result<(), ApplicationError> {
        let mut calendar = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::CalendarNotFound)?;

        calendar.archive();

        self.repository.save(&calendar).await?;

        Ok(())
    }
}
