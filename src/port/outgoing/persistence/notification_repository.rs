use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::model::notification::Notification;

#[async_trait]
trait NotificationRepository {
    fn save(&self, notification: &Notification);

    fn get(&self, uuid: &Uuid) -> Notification;

    fn mark_as_seen(&self, uuid: &Uuid);

    fn mark_as_deleted(&self, uuid: &Uuid);
}