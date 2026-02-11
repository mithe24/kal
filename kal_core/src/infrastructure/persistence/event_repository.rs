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
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
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
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &EventId
    ) -> Result<Option<Event>, RepositoryError> {
        let id_str = id.to_string();

        let model = sqlx::query_as::<_, EventModel>(
            r#"
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                   color, is_all_day, is_cancelled, created_at, updated_at
            FROM events
            WHERE id = ?1
            "#
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match model {
            Some(m) => {
                let event = EventMapper::to_domain(m)
                    .map_err(|e| RepositoryError::DatabaseError(e))?;
                Ok(Some(event))
            }
            None => Ok(None),
        }
    }

    async fn find_by_calendar(
        &self,
        calendar_id: &CalendarId
    ) -> Result<Vec<Event>, RepositoryError> {
        let calendar_id_str = calendar_id.to_string();

        let models = sqlx::query_as::<_, EventModel>(
            r#"
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                   color, is_all_day, is_cancelled, created_at, updated_at
            FROM events
            WHERE calendar_id = ?1
            ORDER BY starts_at
            "#
        )
        .bind(&calendar_id_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        models
            .into_iter()
            .map(|m| EventMapper::to_domain(m)
                .map_err(|e| RepositoryError::DatabaseError(e)))
            .collect()
    }

    async fn find_in_range(
        &self,
        calendar_id: &CalendarId,
        range: &TimeRange,
    ) -> Result<Vec<Event>, RepositoryError> {
        let calendar_id_str = calendar_id.to_string();
        let range_start = range.starts_at().to_rfc3339();
        let range_end = range.ends_at().to_rfc3339();

        let models = sqlx::query_as::<_, EventModel>(
            r#"
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                   color, is_all_day, is_cancelled, created_at, updated_at
            FROM events
            WHERE calendar_id = ?1
              AND is_cancelled = 0
              AND starts_at < ?3
              AND ends_at > ?2
            ORDER BY starts_at
            "#
        )
        .bind(&calendar_id_str)
        .bind(&range_start)
        .bind(&range_end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        models
            .into_iter()
            .map(|m| EventMapper::to_domain(m)
                .map_err(|e| RepositoryError::DatabaseError(e)))
            .collect()
    }

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError> {
        let id_str = id.to_string();
        
        let result = sqlx::query!(
            r#"
                DELETE FROM events WHERE id = ?1
            "#,
            id_str,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
