use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::application::service::clock_service::ClockService;

#[derive(Debug)]
pub struct Notification {
    pub id: Uuid,
    pub message: String,
    pub seen: bool,
    pub deleted: bool,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Notification {
    pub fn new(message: String) -> Self {
        let now = ClockService::now();
        Self {
            id: Uuid::new_v4(),
            message,
            seen: false,
            deleted: false,
            date_created: now,
            last_updated: now,
        }
    }

    pub fn set_as_seen(&mut self) {
        self.seen = true;
        self.last_updated = ClockService::now();
    }

    pub fn set_as_deleted(&mut self) {
        self.seen = true;
        self.deleted = true;
        self.last_updated = ClockService::now();
    }
}