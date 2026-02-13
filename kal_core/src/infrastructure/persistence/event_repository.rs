use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::domain::{
    event::Event,
    repository::{EventRepository, RepositoryError},
    value_objects::{CalendarId, EventId, TimeRange},
};
use super::{models::EventModel, mappers::EventMapper};

pub struct SqliteEventRepository {
    pool: SqlitePool,
}

impl SqliteEventRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn model_to_domain(model: EventModel) -> Result<Event, RepositoryError> {
        EventMapper::to_domain(model)
            .map_err(|e| RepositoryError::MappingError(e.to_string()))
    }

    fn models_to_domain(
        models: Vec<EventModel>
    ) -> Result<Vec<Event>, RepositoryError> {
        models.into_iter().map(Self::model_to_domain).collect()
    }

    fn db_error(e: sqlx::Error) -> RepositoryError {
        RepositoryError::DatabaseError(e.to_string())
    }
}

#[async_trait]
impl EventRepository for SqliteEventRepository {
    async fn save(&self, event: &Event) -> Result<(), RepositoryError> {
        let model = EventMapper::to_model(event);

        sqlx::query!(
            r#"
                INSERT INTO events (
                    id, calendar_id, title, description, starts_at, ends_at,
                    color, is_all_day, is_cancelled, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    description = excluded.description,
                    starts_at = excluded.starts_at,
                    ends_at = excluded.ends_at,
                    color = excluded.color,
                    is_all_day = excluded.is_all_day,
                    is_cancelled = excluded.is_cancelled,
                    updated_at = excluded.updated_at
            "#,
            model.id,
            model.calendar_id,
            model.title,
            model.description,
            model.starts_at,
            model.ends_at,
            model.color,
            model.is_all_day,
            model.is_cancelled,
            model.created_at,
            model.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(Self::db_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self, id: &EventId
    ) -> Result<Option<Event>, RepositoryError> {
        sqlx::query_as::<_, EventModel>("
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                   color, is_all_day, is_cancelled, created_at, updated_at
             FROM events WHERE id = ?
        ")
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(Self::db_error)?
        .map(Self::model_to_domain)
        .transpose()
    }

    async fn find_by_calendar(
        &self,
        calendar_id: &CalendarId
    ) -> Result<Vec<Event>, RepositoryError> {
        let models = sqlx::query_as::<_, EventModel>("
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                   color, is_all_day, is_cancelled, created_at, updated_at
            FROM events WHERE calendar_id = ? ORDER BY starts_at
        ")
        .bind(calendar_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(Self::db_error)?;

        Self::models_to_domain(models)
    }

    async fn find_in_range(
        &self,
        calendar_id: &CalendarId,
        range: &TimeRange,
    ) -> Result<Vec<Event>, RepositoryError> {
        let models = sqlx::query_as::<_, EventModel>("
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                color, is_all_day, is_cancelled, created_at, updated_at
            FROM events
            WHERE calendar_id = ?
                AND is_cancelled = 0
                AND starts_at < ?
                AND ends_at > ?
            ORDER BY starts_at
        ")
        .bind(calendar_id.to_string())
        .bind(range.ends_at().to_rfc3339())
        .bind(range.starts_at().to_rfc3339())
        .fetch_all(&self.pool)
        .await
        .map_err(Self::db_error)?;

        Self::models_to_domain(models)
    }

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError> {
        let event_id = id.to_string();
        let result = sqlx::query!("DELETE FROM events WHERE id = ?", event_id)
            .execute(&self.pool)
            .await
            .map_err(Self::db_error)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
