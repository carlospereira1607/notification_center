use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationEntity {
    pub uuid: Uuid,
    pub message: String,
    pub seen: bool,
    pub deleted: bool,
    pub date_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
