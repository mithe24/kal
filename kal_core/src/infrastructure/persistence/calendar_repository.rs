use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::{
    domain::{
        calendar::Calendar,
        repository::{CalendarRepository, RepositoryError},
        value_objects::CalendarId,
    },
    infrastructure::persistence::models::CalendarModel
};
use super::mappers::CalendarMapper;

pub struct SqliteCalendarRepository {
    pool: SqlitePool,
}

impl SqliteCalendarRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarRepository for SqliteCalendarRepository {
    async fn save(&self, calendar: &Calendar) -> Result<(), RepositoryError> {
        let model = CalendarMapper::to_model(calendar);

        sqlx::query!(
            r#"
                INSERT INTO calendars (
                    id, name, description, is_archived,
                    created_at, updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    description = excluded.description,
                    is_archived = excluded.is_archived,
                    updated_at = excluded.updated_at
            "#,
            model.id,
            model.name,
            model.description,
            model.is_archived,
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
        id: &CalendarId
    ) -> Result<Option<Calendar>, RepositoryError> {
        let id_str = id.to_string();

        let model = sqlx::query_as::<_, CalendarModel>(
            r#"
            SELECT id, name, description, is_archived, created_at, updated_at
            FROM calendars
            WHERE id = ?1
            "#
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match model {
            Some(m) => {
                let calendar = CalendarMapper::to_domain(m)
                    .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                Ok(Some(calendar))
            }
            None => Ok(None),
        }
    }

    async fn find_all_active(&self) -> Result<Vec<Calendar>, RepositoryError> {
        let models = sqlx::query_as::<_, CalendarModel>(
            r#"
            SELECT id, name, description, is_archived, created_at, updated_at
            FROM calendars
            WHERE is_archived = 0
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        models
            .into_iter()
            .map(|m| CalendarMapper::to_domain(m)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string())))
            .collect()
    }

    async fn delete(&self, id: &CalendarId) -> Result<(), RepositoryError> {
        let id_str = id.to_string();

        let result = sqlx::query!(
            r#"
                DELETE FROM calendars WHERE id = ?1
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
