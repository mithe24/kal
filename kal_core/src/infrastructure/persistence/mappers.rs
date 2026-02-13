use chrono::{DateTime, Utc};
use std::{collections::HashMap, str::FromStr};

use crate::domain::{
    value_objects::{CalendarId, EventColor, EventId, Frequency, TimeRange},
    calendar::Calendar,
    event::Event,
    recurrence::{
        ExceptionModification,
        RecurrenceException,
        RecurrenceRule,
        RecurringEvent,
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

fn parse_calendar_id(s: &str) -> MapperResult<CalendarId> {
    CalendarId::from_str(s).map_err(|e| MapperError::InvalidId(e.to_string()))
}

fn parse_event_id(s: &str) -> MapperResult<EventId> {
    EventId::from_str(s).map_err(|e| MapperError::InvalidId(e.to_string()))
}

fn parse_date(s: &str) -> MapperResult<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)
        .map_err(|e| MapperError::InvalidDate(e.to_string()))?
        .with_timezone(&Utc))
}

fn parse_frequency(s: &str) -> MapperResult<Frequency> {
    Frequency::from_str(s).map_err(|e| MapperError::InvalidData(e.to_string()))
}

fn to_bool(value: i64) -> bool {
    value != 0
}

fn from_bool(value: bool) -> i64 {
    if value { 1 } else { 0 }
}

fn parse_event_color(value: i64) -> EventColor {
    EventColor::from(value as u8)
}

fn color_to_i64(color: EventColor) -> i64 {
    u8::from(color) as i64
}

pub struct CalendarMapper;

impl CalendarMapper {
    pub fn to_domain(model: CalendarModel) -> MapperResult<Calendar> {
        Ok(Calendar::with_id(
            parse_calendar_id(&model.id)?,
            model.name,
            model.description,
            to_bool(model.is_archived),
            parse_date(&model.created_at)?,
            parse_date(&model.updated_at)?,
        )?)
    }

    pub fn to_model(calendar: &Calendar) -> CalendarModel {
        CalendarModel {
            id: calendar.calendar_id().to_string(),
            name: calendar.name().to_string(),
            description: calendar.description().clone(),
            is_archived: from_bool(*calendar.is_archived()),
            created_at: calendar.created_at().to_rfc3339(),
            updated_at: calendar.updated_at().to_rfc3339(),
        }
    }
}

pub struct EventMapper;

impl EventMapper {
    pub fn to_domain(model: EventModel) -> MapperResult<Event> {
        let time_range = TimeRange::new(
            parse_date(&model.starts_at)?,
            parse_date(&model.ends_at)?,
        )?;

        Ok(Event::with_id(
            parse_event_id(&model.id)?,
            parse_calendar_id(&model.calendar_id)?,
            model.title,
            model.description,
            time_range,
            parse_event_color(model.color),
            to_bool(model.is_all_day),
            to_bool(model.is_cancelled),
            parse_date(&model.created_at)?,
            parse_date(&model.updated_at)?,
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
            color: color_to_i64(*event.color()),
            is_all_day: from_bool(*event.is_all_day()),
            is_cancelled: from_bool(*event.is_cancelled()),
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
    ) -> MapperResult<RecurringEvent> {
        let base_time_range = TimeRange::new(
            parse_date(&model.starts_at)?,
            parse_date(&model.ends_at)?,
        )?;

        let rule = RecurrenceRule::new(
            parse_frequency(&model.frequency)?,
            model.interval as u32,
            model.until.as_ref().map(|u| parse_date(u)).transpose()?,
        )?;

        let exception_map = exceptions
            .into_iter()
            .map(Self::exception_to_domain)
            .collect::<MapperResult<Vec<_>>>()?
            .into_iter()
            .map(|ex| (*ex.original_starts_at(), ex))
            .collect::<HashMap<_, _>>();

        Ok(RecurringEvent::with_id(
            parse_event_id(&model.id)?,
            parse_calendar_id(&model.calendar_id)?,
            model.title,
            model.description,
            base_time_range,
            rule,
            exception_map,
            parse_event_color(model.color),
            to_bool(model.is_all_day),
            to_bool(model.is_cancelled),
            parse_date(&model.created_at)?,
            parse_date(&model.updated_at)?,
        )?)
    }

    fn exception_to_domain(
        model: RecurrenceExceptionModel
    ) -> MapperResult<RecurrenceException> {
        let original = parse_date(&model.original_starts_at)?;

        if to_bool(model.is_cancelled) {
            return Ok(RecurrenceException::cancelled(original));
        }

        match (model.new_starts_at, model.new_ends_at) {
            (Some(start), Some(end)) => {
                let range = TimeRange::new(
                    parse_date(&start)?,
                    parse_date(&end)?
                )?;
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
        let (new_starts_at, new_ends_at, is_cancelled)
            = match exception.modification() {
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
            color: color_to_i64(*event.color()),
            is_all_day: from_bool(*event.is_all_day()),
            is_cancelled: from_bool(*event.is_cancelled()),
            created_at: event.created_at().to_rfc3339(),
            updated_at: event.updated_at().to_rfc3339(),
        }
    }
}
