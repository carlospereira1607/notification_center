use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct NotificationEntity {
    id: Uuid,
    message: String,
    seen: bool,
    deleted: bool,
    date_created: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}
