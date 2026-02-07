use std::collections::HashMap;

use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{
    error::DomainError,
    value_objects::{CalendarId, EventColor, EventId, Frequency, TimeRange}
};

#[derive(Debug, Clone, Getters)]
pub struct RecurrenceRule {
    #[getset(get = "pub")]
    frequency: Frequency,
    #[getset(get = "pub")]
    interval: u32,
    #[getset(get = "pub")]
    until: Option<DateTime<Utc>>,
}

impl RecurrenceRule {
    pub fn new(
        frequency: Frequency,
        interval: u32,
        until: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if interval == 0 {
            return Err(DomainError::InvalidInterval);
        }
        Ok(Self {
            frequency,
            interval,
            until,
        })
    }
}

#[derive(Debug, Clone, Getters)]
pub struct RecurringEvent {
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
    rule: RecurrenceRule,
    #[getset(get = "pub")]
    exceptions: HashMap<DateTime<Utc>, RecurrenceException>,
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

impl RecurringEvent {
    pub fn new(
        calendar_id: CalendarId,
        title: String,
        description: Option<String>,
        time_range: TimeRange,
        rule: RecurrenceRule,
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
            rule,
            exceptions: HashMap::new(),
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
        rule: RecurrenceRule,
        exceptions: HashMap<DateTime<Utc>, RecurrenceException>,
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
            rule,
            exceptions,
            color,
            is_all_day,
            is_cancelled,
            created_at,
            updated_at,
        }
    }

    pub fn add_exception(&mut self, exception: RecurrenceException) {
        self.exceptions.insert(exception.original_starts_at, exception);
        self.touch();
    }

    pub fn remove_exception(&mut self, original_starts_at: DateTime<Utc>) {
        self.exceptions.remove(&original_starts_at);
        self.touch();
    }

    pub fn restore_occurrence(&mut self, original_starts_at: DateTime<Utc>) {
        self.remove_exception(original_starts_at);
        self.touch();
    }

    pub fn cancel_occurrence(&mut self, original_starts_at: DateTime<Utc>) {
        let exception = RecurrenceException::cancelled(original_starts_at);
        self.add_exception(exception);
    }

    pub fn reschedule_occurrence(
        &mut self,
        original_starts_at: DateTime<Utc>,
        new_time_range: TimeRange,
    ) {
        let exception = RecurrenceException::rescheduled(
            original_starts_at,
            new_time_range
        );
        self.add_exception(exception);
    }

    pub fn cancel(&mut self) {
        self.is_cancelled = true;
        self.touch();
    }

    pub fn restore(&mut self) {
        self.is_cancelled = false;
        self.touch();
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Getters)]
pub struct RecurrenceException {
    #[getset(get = "pub")]
    original_starts_at: DateTime<Utc>,
    #[getset(get = "pub")]
    modification: ExceptionModification,
}

#[derive(Debug, Clone)]
pub enum ExceptionModification {
    Cancelled,
    Rescheduled { new_time_range: TimeRange },
}

impl RecurrenceException {
    pub fn cancelled(original_starts_at: DateTime<Utc>) -> Self {
        Self {
            original_starts_at,
            modification: ExceptionModification::Cancelled,
        }
    }

    pub fn rescheduled(
        original_starts_at: DateTime<Utc>,
        new_time_range: TimeRange,
    ) -> Self {
        Self {
            original_starts_at,
            modification: ExceptionModification::Rescheduled { new_time_range },
        }
    }

    pub fn new_time_range(&self) -> Option<&TimeRange> {
        match &self.modification {
            ExceptionModification::Rescheduled {
                new_time_range,
            } => Some(new_time_range),
            ExceptionModification::Cancelled => None,
        }
    }
}
