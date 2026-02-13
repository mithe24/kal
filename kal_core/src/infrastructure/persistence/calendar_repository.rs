use crate::{
    application::error::ApplicationError,
    domain::{
        calendar::Calendar,
        repository::CalendarRepository,
        value_objects::CalendarId,
    },
};

pub struct CalendarService<R: CalendarRepository> {
    repository: R,
}

impl<R: CalendarRepository> CalendarService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn find_by_id(
        &self,
        calendar_id: &CalendarId,
    ) -> Result<Option<Calendar>, ApplicationError> {
        self.repository
            .find_by_id(calendar_id)
            .await
            .map_err(Into::into)
    }

    pub async fn get_by_id(
        &self,
        calendar_id: &CalendarId,
    ) -> Result<Calendar, ApplicationError> {
        self.find_by_id(calendar_id)
            .await?
            .ok_or(ApplicationError::NotFound(calendar_id.to_string()))
    }

    pub async fn list_active(&self) -> Result<Vec<Calendar>, ApplicationError> {
        self.repository
            .find_all_active()
            .await
            .map_err(Into::into)
    }

    pub async fn verify_active(
        &self,
        calendar_id: &CalendarId,
    ) -> Result<bool, ApplicationError> {
        let calendar = self.get_by_id(calendar_id).await?;

        if *calendar.is_archived() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Check if a calendar exists
    pub async fn exists(
        &self,
        calendar_id: &CalendarId
    ) -> Result<bool, ApplicationError> {
        Ok(self.find_by_id(calendar_id).await?.is_some())
    }

    /// Verify calendar exists (archived or not)
    pub async fn verify_exists(
        &self,
        calendar_id: &CalendarId,
    ) -> Result<(), ApplicationError> {
        if !self.exists(calendar_id).await? {
            return Err(ApplicationError::NotFound(
                format!("Calendar with ID {} not found", calendar_id)
            ));
        }
        Ok(())
    }
}
