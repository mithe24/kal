use crate::{application::error::ApplicationError, domain::{repository::EventRepository, value_objects::{EventId, TimeRange}}};

pub struct UpdateEventTimeRangeCommand {
    id: EventId,
    new_event_time_range: TimeRange,
}

pub struct UpdateEventTimeRangeHandler<R: EventRepository> {
    repository: R,
}

impl<R: EventRepository> UpdateEventTimeRangeHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateEventTimeRangeCommand,
    ) -> Result<(), ApplicationError> {
        let mut event = self
            .repository
            .find_by_id(&command.id)
            .await?
            .ok_or(ApplicationError::EventNotFound)?;

        event.update_time_range(command.new_event_time_range);

        self.repository.save(&event).await?;

        Ok(())
    }
}
