use core::fmt;

use chrono::{DateTime, Utc};
use getset::{Getters};
use uuid::Uuid;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventColor(u8);

impl From<u8> for EventColor {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<EventColor> for u8 {
    fn from(color: EventColor) -> Self {
        color.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Getters)]
pub struct TimeRange {
    #[getset(get = "pub")]
    starts_at: DateTime<Utc>,
    #[getset(get = "pub")]
    ends_at: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(
        starts_at: DateTime<Utc>,
        ends_at: DateTime<Utc>,
    ) -> Result<Self, TimeRangeError> {
        if starts_at >= ends_at {
            Err(TimeRangeError::EndBeforeStart)
        } else {
            Ok(Self { starts_at, ends_at })
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.starts_at < other.ends_at
            && other.starts_at < self.ends_at
    }

    pub fn duration(&self) -> chrono::Duration {
        self.ends_at - self.starts_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frequency::Daily => write!(f, "DAILY"),
            Frequency::Weekly => write!(f, "WEEKLY"),
            Frequency::Monthly => write!(f, "MONTHLY"),
            Frequency::Yearly => write!(f, "YEARLY"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalendarId(Uuid);

impl CalendarId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for CalendarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for CalendarId {
    type Err = uuid::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EventId {
    type Err = uuid::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

// Calendar ID Error
#[derive(Debug, Error)]
pub enum CalendarIdError {
    #[error("Calendar ID cannot be empty")]
    Empty,

    #[error("Invalid calendar ID format: {0}")]
    InvalidFormat(String),

    #[error("Calendar ID must be a valid UUID")]
    InvalidUuid(#[from] uuid::Error),
}

#[derive(Debug, Error)]
pub enum EventIdError {
    #[error("Event ID cannot be empty")]
    Empty,

    #[error("Invalid event ID format: {0}")]
    InvalidFormat(String),

    #[error("Event ID must be a valid UUID")]
    InvalidUuid(#[from] uuid::Error),
}

#[derive(Debug, Error)]
pub enum TimeRangeError {
    #[error("End time must be after start time")]
    EndBeforeStart,
}

#[derive(Debug, Error)]
pub enum FrequencyError {
    #[error("Unknown frequency: {0}")]
    Unknown(String),
}

impl FromStr for Frequency {
    type Err = FrequencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            "monthly" => Ok(Frequency::Monthly),
            "yearly" => Ok(Frequency::Yearly),
            _ => Err(FrequencyError::Unknown(s.to_string())),
        }
    }
}
