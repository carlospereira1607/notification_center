use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::application::service::clock_service::ClockService;

#[derive(Debug, Clone)]
pub struct Notification {
    pub uuid: Uuid,
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
            uuid: Uuid::new_v4(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let notification = Notification::new("test".to_string());
        assert_eq!(notification.message, "test");
        assert_eq!(notification.seen, false);
        assert_eq!(notification.deleted, false);
    }

    #[test]
    fn test_set_as_seen() {
        let mut notification = Notification::new("test".to_string());
        notification.set_as_seen();
        assert_eq!(notification.seen, true);
    }

    #[test]
    fn test_set_as_deleted() {
        let mut notification = Notification::new("test".to_string());
        notification.set_as_deleted();
        assert_eq!(notification.seen, true);
        assert_eq!(notification.deleted, true);
    }

}