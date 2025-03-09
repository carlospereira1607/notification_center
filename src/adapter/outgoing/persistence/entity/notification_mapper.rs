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

impl From<NotificationEntity> for Notification {
    fn from(notification_entity: NotificationEntity) -> Self {
        Self {
            uuid: notification_entity.uuid,
            message: notification_entity.message,
            seen: notification_entity.seen,
            deleted: notification_entity.deleted,
            date_created: notification_entity.date_created,
            last_updated: notification_entity.last_updated,
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

    #[test]
    fn test_notification_entity_to_notification_mapping() {
        let expected_notification = Notification::new("some message".to_string());

        let notification_entity = NotificationEntity {
            uuid: expected_notification.uuid,
            message: expected_notification.message,
            seen: false,
            deleted: false,
            date_created: expected_notification.date_created,
            last_updated: expected_notification.last_updated,
        };

        let result: Notification = notification_entity.clone().into();

        assert_eq!(result.uuid, expected_notification.uuid);
        assert_eq!(result.message, "some message");
        assert_eq!(result.seen, expected_notification.seen);
        assert_eq!(result.deleted, expected_notification.deleted);
        assert_eq!(result.date_created, expected_notification.date_created);
        assert_eq!(result.last_updated, expected_notification.last_updated);
    }
}