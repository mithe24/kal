use async_trait::async_trait;
use sqlx::{SqlitePool, Transaction, Sqlite};
use crate::domain::{
    recurrence::RecurringEvent,
    value_objects::{CalendarId, EventId},
    repository::{RecurringEventRepository, RepositoryError},
};
use super::{
    mappers::RecurrenceMapper,
    models::{RecurrenceModel, RecurrenceExceptionModel},
};

pub struct SqliteRecurringEventRepository {
    pool: SqlitePool,
}

impl SqliteRecurringEventRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn begin_tx(
        &self
    ) -> Result<Transaction<'_, Sqlite>, RepositoryError> {
        self.pool.begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    async fn save_recurrence(
        tx: &mut Transaction<'_, Sqlite>,
        model: &super::models::RecurrenceModel,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO recurrences (
                    id, calendar_id, title, description, starts_at, ends_at,
                    frequency, interval, until, color, is_all_day, is_cancelled,
                    created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
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
        .execute(&mut **tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn save_exceptions(
        tx: &mut Transaction<'_, Sqlite>,
        event: &RecurringEvent,
    ) -> Result<(), RepositoryError> {
        let event_id = event.event_id().to_string();
        sqlx::query!(
            "DELETE FROM recurrence_exceptions WHERE recurrence_id = ?",
            event_id,
        )
            .execute(&mut **tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        for exception in event.exceptions().values() {
            let ex_model = RecurrenceMapper::exception_to_model(
                exception,
                event.event_id(),
            );

            sqlx::query!(
                r#"
                    INSERT INTO recurrence_exceptions (
                        recurrence_id, original_starts_at, new_starts_at,
                        new_ends_at, is_cancelled
                    )
                    VALUES ($1, $2, $3, $4, $5)
                "#,
                ex_model.recurrence_id,
                ex_model.original_starts_at,
                ex_model.new_starts_at,
                ex_model.new_ends_at,
                ex_model.is_cancelled,
            )
            .execute(&mut **tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    async fn load_exceptions(
        &self,
        recurrence_id: &str,
    ) -> Result<Vec<RecurrenceExceptionModel>, RepositoryError> {
        sqlx::query_as::<_, RecurrenceExceptionModel>(
            "SELECT id, recurrence_id, original_starts_at, new_starts_at,
                    new_ends_at, is_cancelled
             FROM recurrence_exceptions WHERE recurrence_id = ?"
        )
        .bind(recurrence_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    fn to_domain(
        model: RecurrenceModel,
        exceptions: Vec<RecurrenceExceptionModel>,
    ) -> Result<RecurringEvent, RepositoryError> {
        RecurrenceMapper::to_domain(model, exceptions)
            .map_err(|e| RepositoryError::MappingError(e.to_string()))
    }
}

#[async_trait]
impl RecurringEventRepository for SqliteRecurringEventRepository {
    async fn save(
        &self,
        event: &RecurringEvent
    ) -> Result<(), RepositoryError> {
        let mut tx = self.begin_tx().await?;
        let model = RecurrenceMapper::to_model(event);

        Self::save_recurrence(&mut tx, &model).await?;
        Self::save_exceptions(&mut tx, event).await?;

        tx.commit()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &EventId
    ) -> Result<RecurringEvent, RepositoryError> {
        let model = sqlx::query_as::<_, RecurrenceModel>("
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                    frequency, interval, until, color, is_all_day, is_cancelled,
                    created_at, updated_at
             FROM recurrences WHERE id = ?
        ")
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or(RepositoryError::NotFound)?;

        let exceptions = self.load_exceptions(&model.id).await?;
        Self::to_domain(model, exceptions)
    }

    async fn find_by_calendar(
        &self,
        calendar_id: &CalendarId
    ) -> Result<Vec<RecurringEvent>, RepositoryError> {
        let models = sqlx::query_as::<_, RecurrenceModel>("
            SELECT id, calendar_id, title, description, starts_at, ends_at,
                    frequency, interval, until, color, is_all_day, is_cancelled,
                    created_at, updated_at
             FROM recurrences WHERE calendar_id = ? ORDER BY starts_at
        ")
        .bind(calendar_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for model in models {
            let exceptions = self.load_exceptions(&model.id).await?;
            result.push(Self::to_domain(model, exceptions)?);
        }

        Ok(result)
    }

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError> {
        let event_id = id.to_string();
        let result = sqlx::query!(
            "DELETE FROM recurrences WHERE id = ?",
            event_id,
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
