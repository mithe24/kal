use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::value_objects::{CalendarId, EventColor, EventId, TimeRange};

#[derive(Debug, Clone, Getters)]
pub struct Event {
    #[getset(get = "pub")]
    id: EventId,
    #[getset(get = "pub")]
    calendar_id: CalendarId,
    #[getset(get = "pub")]
    title: String,
    #[getset(get = "pub")]
    description: Option<String>,
    #[getset(get = "pub")]
    time_range: TimeRange,
    #[getset(get = "pub")]
    color: EventColor,
    #[getset(get = "pub")]
    is_all_day: bool,
    #[getset(get = "pub")]
    is_cancelled: bool,
    #[getset(get = "pub")]
    created_at: DateTime<Utc>,
    #[getset(get = "pub")]
    updated_at: DateTime<Utc>,
}

impl Event {
    pub fn new(
        calendar_id: CalendarId,
        title: String,
        description: Option<String>,
        time_range: TimeRange,
        color: EventColor,
        is_all_day: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EventId::new(),
            calendar_id,
            title,
            description,
            time_range,
            color,
            is_all_day,
            is_cancelled: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn with_id(
        id: EventId,
        calendar_id: CalendarId,
        title: String,
        description: Option<String>,
        time_range: TimeRange,
        color: EventColor,
        is_all_day: bool,
        is_cancelled: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            calendar_id,
            title,
            description,
            time_range,
            color,
            is_all_day,
            is_cancelled,
            created_at,
            updated_at,
        }
    }
    
    pub fn cancel(&mut self) {
        self.is_cancelled = true;
        self.updated_at = Utc::now();
    }

    pub fn restore(&mut self) {
        self.is_cancelled = false;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn update_time_range(&mut self, time_range: TimeRange) {
        self.time_range = time_range;
        self.updated_at = Utc::now();
    }

    pub fn update_color(&mut self, color: EventColor) {
        self.color = color;
        self.updated_at = Utc::now();
    }

    pub fn overlaps_with(&self, other: &Event) -> bool {
        !self.is_cancelled 
            && !other.is_cancelled 
            && self.calendar_id == other.calendar_id
            && self.time_range.overlaps(other.time_range())
    }
}
