use crate::adapter::outgoing::persistence::entity::notification_entity::NotificationEntity;
use crate::domain::model::notification::Notification;

impl From<Notification> for NotificationEntity {
    fn from(notification: Notification) -> Self {
        Self {
            uuid: notification.uuid,
            content: notification.content,
            seen: notification.seen,
            deleted: notification.deleted,
            date_created: notification.date_created,
            last_updated: notification.last_updated,
        }
    }
}