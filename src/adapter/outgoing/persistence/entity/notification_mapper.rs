use crate::adapter::outgoing::persistence::entity::notification_entity::NotificationEntity;
use crate::domain::model::notification::Notification;

impl From<Notification> for NotificationEntity {
    fn from(notification: Notification) -> Self {
        Self {
            uuid: notification.uuid,
            message: notification.message,
            seen: notification.seen,
            deleted: notification.deleted,
            date_created: notification.date_created,
            last_updated: notification.last_updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::notification::Notification;

    #[test]
    fn test_notification_to_entity_mapping() {
        let notification = Notification::new("some message".to_string());
        let entity: NotificationEntity = notification.clone().into();

        assert_eq!(entity.uuid, notification.uuid);
        assert_eq!(entity.message, notification.message);
        assert_eq!(entity.seen, notification.seen);
        assert_eq!(entity.deleted, notification.deleted);
        assert_eq!(entity.date_created, notification.date_created);
        assert_eq!(entity.last_updated, notification.last_updated);
    }
}