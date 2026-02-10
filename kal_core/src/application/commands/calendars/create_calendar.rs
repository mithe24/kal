use crate::{
    application::error::ApplicationError,
    domain::{
        calendar::Calendar,
        repository::CalendarRepository,
        value_objects::CalendarId
    }
};

pub struct CreateCalendarCommand {
    name: String,
    description: Option<String>,
}


pub struct CreateCalendarHandler<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> CreateCalendarHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateCalendarCommand,
    ) -> Result<CalendarId, ApplicationError> {
        let calendar = Calendar::new(command.name, command.description);

        let calendar_id = calendar.id().clone();

        self.repository.save(&calendar).await?;

        Ok(calendar_id)
    }
}
