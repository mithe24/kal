use std::result;

use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::domain::{
    recurrence::RecurringEvent,
    repository::{RecurringEventRepository, RepositoryError, Result},
    value_objects::{CalendarId, EventId},
};
use super::{
    models::{RecurrenceModel, RecurrenceExceptionModel},
    mappers::RecurrenceMapper,
};

pub struct SqliteRecurringEventRepository {
    pool: SqlitePool,
}

impl SqliteRecurringEventRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecurringEventRepository for SqliteRecurringEventRepository {
    async fn save(&self, event: &RecurringEvent) -> Result<()> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let model = RecurrenceMapper::to_model(event);

        sqlx::query!(
            r#"
                INSERT INTO recurrences (
                    id, calendar_id, title, description, starts_at, ends_at,
                    frequency, interval, until, color, is_all_day, is_cancelled,
                    created_at, updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
                ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    description = excluded.description,
                    starts_at = excluded.starts_at,
                    ends_at = excluded.ends_at,
                    frequency = excluded.frequency,
                    interval = excluded.interval,
                    until = excluded.until,
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
            model.frequency,
            model.interval,
            model.until,
            model.color,
            model.is_all_day,
            model.is_cancelled,
            model.created_at,
            model.updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query!(
            r#"
                DELETE FROM recurrence_exceptions WHERE recurrence_id = ?1
            "#,
            model.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        for exception in event.exceptions().values() {
            let ex_model = RecurrenceMapper::exception_to_model(
                exception,
                event.id(),
            );

            sqlx::query!(
                r#"
                    INSERT INTO recurrence_exceptions (
                        recurrence_id, original_starts_at, new_starts_at,
                        new_ends_at, is_cancelled
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                ex_model.recurrence_id,
                ex_model.original_starts_at,
                ex_model.new_starts_at,
                ex_model.new_ends_at,
                ex_model.is_cancelled,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_calendar(
        &self,
        calendar_id: &CalendarId
    ) -> Result<Vec<RecurringEvent>> {
        let calendar_id_str = calendar_id.to_string();

        let models = sqlx::query_as::<_, RecurrenceModel>(
            r#"
                SELECT id, calendar_id, title, description, starts_at, ends_at,
                       frequency, interval, until, color, is_all_day,
                       is_cancelled, created_at, updated_at
                FROM recurrences
                WHERE calendar_id = ?1
                ORDER BY starts_at
            "#
        )
        .bind(&calendar_id_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();

        for model in models {
            let exceptions = sqlx::query_as::<_, RecurrenceExceptionModel>(
                r#"
                    SELECT id, recurrence_id, original_starts_at, new_starts_at,
                           new_ends_at, is_cancelled
                    FROM recurrence_exceptions
                    WHERE recurrence_id = ?1
                "#
            )
            .bind(&model.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

            let event = RecurrenceMapper::to_domain(model, exceptions)
                .map_err(|e| RepositoryError::DatabaseError(e))?;

            result.push(event);
        }

        Ok(result)
    }

    async fn find_by_id(&self, id: &EventId) -> Result<RecurringEvent> {
        let id_str = id.to_string();

        let model = sqlx::query_as::<_, RecurrenceModel>(
            r#"
                SELECT id, calendar_id, title, description, starts_at, ends_at,
                       frequency, interval, until, color, is_all_day,
                       is_cancelled,
                       created_at, updated_at
                FROM recurrences
                WHERE id = ?1
            "#
        )
            .bind(&id_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let model = match model {
            Some(m) => m,
            None => return Err(RepositoryError::NotFound),
        };

        let exceptions = sqlx::query_as::<_, RecurrenceExceptionModel>(
            r#"
                SELECT recurrence_id, original_starts_at, new_starts_at,
                       new_ends_at, is_cancelled
                FROM recurrence_exceptions
                WHERE recurrence_id = ?1
            "#
        )
            .bind(&model.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Map to domain
        let event = RecurrenceMapper::to_domain(model, exceptions)
            .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(event)
    }

    async fn delete(&self, id: &EventId) -> Result<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM recurrences WHERE id = ?1
            "#,
            id,
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
