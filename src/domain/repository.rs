use async_trait::async_trait;
use super::{
    calendar::Calendar,
    event::Event,
    recurrence::RecurringEvent,
    value_objects::{CalendarId, EventId, TimeRange},
};

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    async fn save(&self, calendar: &Calendar) -> Result<()>;
    async fn find_by_id(&self, id: &CalendarId) -> Result<Option<Calendar>>;
    async fn find_all_active(&self) -> Result<Vec<Calendar>>;
    async fn delete(&self, id: &CalendarId) -> Result<()>;
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn save(&self, event: &Event) -> Result<()>;
    async fn find_by_id(&self, id: &EventId) -> Result<Option<Event>>;
    async fn find_by_calendar(&self, calendar_id: &CalendarId) -> Result<Vec<Event>>;
    async fn find_in_range(&self, calendar_id: &CalendarId, range: &TimeRange) -> Result<Vec<Event>>;
    async fn delete(&self, id: &EventId) -> Result<()>;
}

#[async_trait]
pub trait RecurringEventRepository: Send + Sync {
    async fn save(&self, event: &RecurringEvent) -> Result<()>;
    async fn find_by_calendar(&self, calendar_id: &CalendarId) -> Result<Vec<RecurringEvent>>;
    async fn find_by_id(&self, event_id: &CalendarId) -> Result<RecurringEvent>;
    async fn delete(&self, id: &EventId) -> Result<()>;
}
