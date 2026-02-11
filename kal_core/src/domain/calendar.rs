use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{error::DomainError, value_objects::CalendarId};

#[derive(Debug, Clone, Getters)]
pub struct Calendar {
    #[getset(get = "pub")]
    calendar_id: CalendarId,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    description: Option<String>,
    #[getset(get = "pub")]
    is_archived: bool,
    #[getset(get = "pub")]
    created_at: DateTime<Utc>,
    #[getset(get = "pub")]
    updated_at: DateTime<Utc>,
}

impl Calendar {
    pub fn new(
        name: String,
        description: Option<String>
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        if name.is_empty() {
            Err(DomainError::EmptyName)
        } else {
            Ok(Self {
                calendar_id: CalendarId::new(),
                name,
                description,
                is_archived: false,
                created_at: now,
                updated_at: now,
            })
        }
    }

    pub fn with_id(
        calendar_id: CalendarId,
        name: String,
        description: Option<String>,
        is_archived: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            Err(DomainError::EmptyName)
        } else {
            Ok(Self {
                calendar_id,
                name,
                description,
                is_archived,
                created_at,
                updated_at,
            })
        }
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
        self.touch();
    }

    pub fn unarchive(&mut self) {
        self.is_archived = false;
        self.touch();
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.touch();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.touch();
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
