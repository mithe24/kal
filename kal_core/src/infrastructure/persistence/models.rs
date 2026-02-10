use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CalendarModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_archived: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct EventModel {
    pub id: String,
    pub calendar_id: String,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: String,
    pub ends_at: String,
    pub color: i64,
    pub is_all_day: i64,
    pub is_cancelled: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct RecurrenceModel {
    pub id: String,
    pub calendar_id: String,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: String,
    pub ends_at: String,
    pub frequency: String,
    pub interval: i64,
    pub until: Option<String>,
    pub color: i64,
    pub is_all_day: i64,
    pub is_cancelled: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct RecurrenceExceptionModel {
    pub recurrence_id: String,
    pub original_starts_at: String,
    pub new_starts_at: Option<String>,
    pub new_ends_at: Option<String>,
    pub is_cancelled: i64,
}
