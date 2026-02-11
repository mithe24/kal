use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{
    error::DomainError,
    value_objects::{CalendarId, EventColor, EventId, TimeRange}
};

#[derive(Debug, Clone, Getters)]
pub struct Event {
    #[getset(get = "pub")]
    event_id: EventId,
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
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        if title.is_empty() {
            Err(DomainError::EmptyTitle)
        } else {
            Ok(Self {
                event_id: EventId::new(),
                calendar_id,
                title,
                description,
                time_range,
                color,
                is_all_day,
                is_cancelled: false,
                created_at: now,
                updated_at: now,
            })
        }
    }
    
    pub fn with_id(
        event_id: EventId,
        calendar_id: CalendarId,
        title: String,
        description: Option<String>,
        time_range: TimeRange,
        color: EventColor,
        is_all_day: bool,
        is_cancelled: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if title.is_empty() {
            Err(DomainError::EmptyTitle)
        } else {
            Ok(Self {
                event_id,
                calendar_id,
                title,
                description,
                time_range,
                color,
                is_all_day,
                is_cancelled,
                created_at,
                updated_at,
            })
        }
    }
    
    pub fn cancel(&mut self) {
        self.is_cancelled = true;
        self.touch();
    }

    pub fn restore(&mut self) {
        self.is_cancelled = false;
        self.touch();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.touch();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.touch();
    }

    pub fn update_time_range(&mut self, time_range: TimeRange) {
        self.time_range = time_range;
        self.touch();
    }

    pub fn update_color(&mut self, color: EventColor) {
        self.color = color;
        self.touch();
    }

    pub fn overlaps_with(&self, other: &Event) -> bool {
        !self.is_cancelled 
            && !other.is_cancelled 
            && self.calendar_id == other.calendar_id
            && self.time_range.overlaps(other.time_range())
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
