use crate::{
    application::error::ApplicationError,
    domain::{
        event::Event,
        repository::EventRepository,
        value_objects::{CalendarId, EventColor, EventId, TimeRange}
    }
};

pub struct CreateEventCommand {
    calendar_id: CalendarId,
    title: String,
    description: Option<String>,
    time_range: TimeRange,
    color: EventColor,
    is_all_day: bool,
}

pub struct CreateEventHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> CreateEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateEventCommand,
    ) -> Result<EventId, ApplicationError> {
        let event = Event::new(
            command.calendar_id,
            command.title,
            command.description,
            command.time_range,
            command.color,
            command.is_all_day
        )?;

        let event_id = event.event_id().clone();

        self.repository.save(&event).await?;

        Ok(event_id)
    }
}
