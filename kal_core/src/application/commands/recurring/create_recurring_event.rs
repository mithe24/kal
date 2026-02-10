use crate:: {
    application::error::ApplicationError,
    domain::{
        recurrence::{RecurrenceRule, RecurringEvent},
        repository::RecurringEventRepository,
        value_objects::{CalendarId, EventColor, EventId, TimeRange},
    },
};

pub struct CreateRecurringEventCommand {
    calendar_id: CalendarId,
    title: String,
    description: Option<String>,
    time_range: TimeRange,
    rule: RecurrenceRule,
    color: EventColor,
    is_all_day: bool,
}

pub struct CreateRecurringEventHandler<R: RecurringEventRepository> {
    repository: R,
}

impl<R: RecurringEventRepository> CreateRecurringEventHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateRecurringEventCommand
    ) -> Result<EventId, ApplicationError> {
        let event = RecurringEvent::new(
            command.calendar_id,
            command.title,
            command.description,
            command.time_range,
            command.rule,
            command.color,
            command.is_all_day,
        );

        let event_id = event.id().clone();

        self.repository.save(&event).await?;

        Ok(event_id)
    } 
}
