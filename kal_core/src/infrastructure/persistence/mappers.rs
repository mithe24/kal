use chrono::{DateTime, Utc};
use crate::domain::{
    calendar::Calendar,
    event::Event,
    recurrence::{
        ExceptionModification,
        RecurrenceException,
        RecurrenceRule,
        RecurringEvent,
    },
    value_objects::{
        CalendarId,
        EventColor,
        EventId,
        Frequency,
        TimeRange,
    },
};
use super::models::{
    CalendarModel,
    EventModel,
    RecurrenceModel,
    RecurrenceExceptionModel
};
use std::{collections::HashMap, str::FromStr};

pub struct CalendarMapper;

impl CalendarMapper {
    pub fn to_domain(model: CalendarModel) -> Result<Calendar, String> {
        let id = CalendarId::from_str(&model.id)
            .map_err(|e| format!("Invalid calendar ID: {}", e))?;

        let created_at = DateTime::parse_from_rfc3339(&model.created_at)
            .map_err(|e| format!("Invalid created_at: {}", e))?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&model.updated_at)
            .map_err(|e| format!("Invalid updated_at: {}", e))?
            .with_timezone(&Utc);

        Ok(Calendar::with_id(
            id,
            model.name,
            model.description,
            model.is_archived != 0,
            created_at,
            updated_at,
        ))
    }

    pub fn to_model(calendar: &Calendar) -> CalendarModel {
        CalendarModel {
            id: calendar.id().to_string(),
            name: calendar.name().to_string(),
            description: calendar.description().clone(),
            is_archived: if *calendar.is_archived() { 1 } else { 0 },
            created_at: calendar.created_at().to_rfc3339(),
            updated_at: calendar.updated_at().to_rfc3339(),
        }
    }
}

pub struct EventMapper;

impl EventMapper {
    pub fn to_domain(model: EventModel) -> Result<Event, String> {
        let id = EventId::from_str(&model.id)
            .map_err(|e| format!("Invalid event ID: {}", e))?;

        let calendar_id = CalendarId::from_str(&model.calendar_id)
            .map_err(|e| format!("Invalid calendar ID: {}", e))?;

        let starts_at = DateTime::parse_from_rfc3339(&model.starts_at)
            .map_err(|e| format!("Invalid starts_at: {}", e))?
            .with_timezone(&Utc);

        let ends_at = DateTime::parse_from_rfc3339(&model.ends_at)
            .map_err(|e| format!("Invalid ends_at: {}", e))?
            .with_timezone(&Utc);

        let created_at = DateTime::parse_from_rfc3339(&model.created_at)
            .map_err(|e| format!("Invalid created_at: {}", e))?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&model.updated_at)
            .map_err(|e| format!("Invalid updated_at: {}", e))?
            .with_timezone(&Utc);

        let time_range = TimeRange::new(starts_at, ends_at)
            .map_err(|e| e.to_string())?;

        let color = EventColor::from(model.color as u8);

        Ok(Event::with_id(
            id,
            calendar_id,
            model.title,
            model.description,
            time_range,
            color,
            model.is_all_day != 0,
            model.is_cancelled != 0,
            created_at,
            updated_at,
        ))
    }
    
    pub fn to_model(event: &Event) -> EventModel {
        EventModel {
            id: event.id().to_string(),
            calendar_id: event.calendar_id().to_string(),
            title: event.title().to_string(),
            description: event.description().clone(),
            starts_at: event.time_range().starts_at().to_rfc3339(),
            ends_at: event.time_range().ends_at().to_rfc3339(),
            color: u8::from(*event.color()) as i64,
            is_all_day: if *event.is_all_day() { 1 } else { 0 },
            is_cancelled: if *event.is_cancelled() { 1 } else { 0 },
            created_at: event.created_at().to_rfc3339(),
            updated_at: event.updated_at().to_rfc3339(),
        }
    }
}

pub struct RecurrenceMapper;

impl RecurrenceMapper {
    pub fn to_domain(
        model: RecurrenceModel,
        exceptions: Vec<RecurrenceExceptionModel>,
    ) -> Result<RecurringEvent, String> {
        let id = EventId::from_str(&model.id)
            .map_err(|e| format!("Invalid event ID: {}", e))?;

        let calendar_id = CalendarId::from_str(&model.calendar_id)
            .map_err(|e| format!("Invalid calendar ID: {}", e))?;

        let starts_at = DateTime::parse_from_rfc3339(&model.starts_at)
            .map_err(|e| format!("Invalid starts_at: {}", e))?
            .with_timezone(&Utc);

        let ends_at = DateTime::parse_from_rfc3339(&model.ends_at)
            .map_err(|e| format!("Invalid ends_at: {}", e))?
            .with_timezone(&Utc);

        let created_at = DateTime::parse_from_rfc3339(&model.created_at)
            .map_err(|e| format!("Invalid created_at: {}", e))?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&model.updated_at)
            .map_err(|e| format!("Invalid updated_at: {}", e))?
            .with_timezone(&Utc);

        let base_time_range = TimeRange::new(starts_at, ends_at)
            .map_err(|e| e.to_string())?;

        let color = EventColor::from(model.color as u8);

        let frequency = Frequency::from_str(&model.frequency)
            .map_err(|e| e.to_string())?;

        let until = if let Some(until_str) = model.until {
            Some(DateTime::parse_from_rfc3339(&until_str)
                .map_err(|e| format!("Invalid until: {}", e))?
                .with_timezone(&Utc))
        } else {
            None
        };

        let recurrence_rule = RecurrenceRule::new(
            frequency,
            model.interval as u32,
            until,
        ).map_err(|e| e.to_string())?;

        let exception_map = exceptions
            .into_iter()
            .map(Self::exception_to_domain)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|ex| (*ex.original_starts_at(), ex))
            .collect::<HashMap<DateTime<Utc>, RecurrenceException>>();

        Ok(RecurringEvent::with_id(
            id,
            calendar_id,
            model.title,
            model.description,
            base_time_range,
            recurrence_rule,
            exception_map,
            color,
            model.is_all_day != 0,
            model.is_cancelled != 0,
            created_at,
            updated_at,
        ))
    }

    pub fn to_model(event: &RecurringEvent) -> RecurrenceModel {
        RecurrenceModel {
            id: event.id().to_string(),
            calendar_id: event.calendar_id().to_string(),
            title: event.title().to_string(),
            description: event.description().clone(),
            starts_at: event.time_range().starts_at().to_rfc3339(),
            ends_at: event.time_range().ends_at().to_rfc3339(),
            frequency: event.rule().frequency().to_string(),
            interval: *event.rule().interval() as i64,
            until: event.rule().until().map(|dt| dt.to_rfc3339()),
            color: u8::from(*event.color()) as i64,
            is_all_day: if *event.is_all_day() { 1 } else { 0 },
            is_cancelled: if *event.is_cancelled() { 1 } else { 0 },
            created_at: event.created_at().to_rfc3339(),
            updated_at: event.updated_at().to_rfc3339(),
        }
    }

    fn exception_to_domain(
        model: RecurrenceExceptionModel,
    ) -> Result<RecurrenceException, String> {
        let original_starts_at = DateTime::parse_from_rfc3339(
            &model.original_starts_at
        )
            .map_err(|e| format!("Invalid original_starts_at: {}", e))?
            .with_timezone(&Utc);

        if model.is_cancelled != 0 {
            Ok(RecurrenceException::cancelled(original_starts_at))
        } else if let (Some(new_starts), Some(new_ends)) = 
            (model.new_starts_at, model.new_ends_at) {
            let starts = DateTime::parse_from_rfc3339(&new_starts)
                .map_err(|e| format!("Invalid new_starts_at: {}", e))?
                .with_timezone(&Utc);

            let ends = DateTime::parse_from_rfc3339(&new_ends)
                .map_err(|e| format!("Invalid new_ends_at: {}", e))?
                .with_timezone(&Utc);

            let new_time_range = TimeRange::new(starts, ends)
                .map_err(|e| e.to_string())?;

            Ok(RecurrenceException::rescheduled(
                original_starts_at,
                new_time_range,
            ))
        } else {
            Err(
                "Exception must be either cancelled or have new time range"
                .to_string()
            )
        }
    }

    pub fn exception_to_model(
        exception: &RecurrenceException,
        recurrence_id: &EventId,
    ) -> RecurrenceExceptionModel {
        let (new_starts_at, new_ends_at, is_cancelled) = 
            match exception.modification() {
                ExceptionModification::Cancelled => (None, None, 1),
                ExceptionModification::Rescheduled { new_time_range } => (
                    Some(new_time_range.starts_at().to_rfc3339()),
                    Some(new_time_range.ends_at().to_rfc3339()),
                    0,
                ),
            };

        RecurrenceExceptionModel {
            recurrence_id: recurrence_id.to_string(),
            original_starts_at: exception.original_starts_at().to_rfc3339(),
            new_starts_at,
            new_ends_at,
            is_cancelled,
        }
    }
}
