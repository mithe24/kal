use chrono::{DateTime, Utc};

use std::{collections::HashMap, str::FromStr};

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
    RecurrenceExceptionModel,
};

use crate::infrastructure::persistence::error::MapperError;

type MapperResult<T> = Result<T, MapperError>;


// ======================================================
// Calendar
// ======================================================

pub struct CalendarMapper;

impl CalendarMapper {
    pub fn to_domain(model: CalendarModel) -> MapperResult<Calendar> {
        let id = CalendarId::from_str(&model.id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let created_at = parse_date(&model.created_at)?;
        let updated_at = parse_date(&model.updated_at)?;

        Ok(Calendar::with_id(
            id,
            model.name,
            model.description,
            model.is_archived != 0,
            created_at,
            updated_at,
        )?)
    }

    pub fn to_model(calendar: &Calendar) -> CalendarModel {
        CalendarModel {
            id: calendar.calendar_id().to_string(),
            name: calendar.name().to_string(),
            description: calendar.description().clone(),
            is_archived: if *calendar.is_archived() { 1 } else { 0 },
            created_at: calendar.created_at().to_rfc3339(),
            updated_at: calendar.updated_at().to_rfc3339(),
        }
    }
}


// ======================================================
// Event
// ======================================================

pub struct EventMapper;

impl EventMapper {
    pub fn to_domain(model: EventModel) -> MapperResult<Event> {
        let _id = CalendarId::from_str(&model.id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let event_id = EventId::from_str(&model.id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let calendar_id = CalendarId::from_str(&model.calendar_id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let starts_at = parse_date(&model.starts_at)?;
        let ends_at = parse_date(&model.ends_at)?;

        let created_at = parse_date(&model.created_at)?;
        let updated_at = parse_date(&model.updated_at)?;

        let time_range = TimeRange::new(starts_at, ends_at)?;

        let color = EventColor::from(model.color as u8);

        Ok(Event::with_id(
            event_id,
            calendar_id,
            model.title,
            model.description,
            time_range,
            color,
            model.is_all_day != 0,
            model.is_cancelled != 0,
            created_at,
            updated_at,
        )?)
    }

    pub fn to_model(event: &Event) -> EventModel {
        EventModel {
            id: event.event_id().to_string(),
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


// ======================================================
// Recurring
// ======================================================

pub struct RecurrenceMapper;

impl RecurrenceMapper {
    pub fn to_domain(
        model: RecurrenceModel,
        exceptions: Vec<RecurrenceExceptionModel>,
    ) -> MapperResult<RecurringEvent> {

        let event_id = EventId::from_str(&model.id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let calendar_id = CalendarId::from_str(&model.calendar_id)
            .map_err(|e| MapperError::InvalidId(e.to_string()))?;

        let starts_at = parse_date(&model.starts_at)?;
        let ends_at = parse_date(&model.ends_at)?;

        let created_at = parse_date(&model.created_at)?;
        let updated_at = parse_date(&model.updated_at)?;

        let base_time_range = TimeRange::new(starts_at, ends_at)?;

        let color = EventColor::from(model.color as u8);

        let frequency = Frequency::from_str(&model.frequency)?;

        let until = match model.until {
            Some(u) => Some(parse_date(&u)?),
            None => None,
        };

        let rule = RecurrenceRule::new(
            frequency,
            model.interval as u32,
            until,
        )?;

        let exception_map = exceptions
            .into_iter()
            .map(Self::exception_to_domain)
            .collect::<MapperResult<Vec<_>>>()?
            .into_iter()
            .map(|ex| (*ex.original_starts_at(), ex))
            .collect::<HashMap<_, _>>();

        Ok(RecurringEvent::with_id(
            event_id,
            calendar_id,
            model.title,
            model.description,
            base_time_range,
            rule,
            exception_map,
            color,
            model.is_all_day != 0,
            model.is_cancelled != 0,
            created_at,
            updated_at,
        )?)
    }


    // ==================================================
    // Exceptions
    // ==================================================

    fn exception_to_domain(
        model: RecurrenceExceptionModel,
    ) -> MapperResult<RecurrenceException> {

        let original = parse_date(&model.original_starts_at)?;

        if model.is_cancelled != 0 {
            return Ok(RecurrenceException::cancelled(original));
        }

        match (model.new_starts_at, model.new_ends_at) {
            (Some(start), Some(end)) => {
                let starts = parse_date(&start)?;
                let ends = parse_date(&end)?;

                let range = TimeRange::new(starts, ends)?;

                Ok(RecurrenceException::rescheduled(original, range))
            }

            _ => Err(MapperError::InvalidData(
                "Exception must be cancelled or rescheduled".into(),
            )),
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


    pub fn to_model(event: &RecurringEvent) -> RecurrenceModel {
        RecurrenceModel {
            id: event.event_id().to_string(),
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
}


// ======================================================
// Helpers
// ======================================================

fn parse_date(s: &str) -> MapperResult<DateTime<Utc>> {
    Ok(
        DateTime::parse_from_rfc3339(s)
            .map_err(|e| MapperError::InvalidDate(e.to_string()))?
            .with_timezone(&Utc)
    )
}
