use crate::domain::model::notification::Notification;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    uuid: String,
    message: String,
    seen: bool,
    deleted: bool,
}

impl From<Notification> for NotificationResponse {
    fn from(notification: Notification) -> Self {
        Self {
            uuid: notification.uuid.to_string(),
            message: notification.message,
            seen: notification.seen,
            deleted: notification.deleted,
        }
    }
}
