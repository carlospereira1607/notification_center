use crate::application::error::ServiceError;
use crate::domain::model::notification::Notification;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait NotificationServicePort: Send + Sync {
    async fn create_notification(&self, message: String) -> Result<Notification, ServiceError>;
    async fn get_notification(&self, uuid: Uuid) -> Result<Notification, ServiceError>;
    async fn list_notifications(&self) -> Result<Vec<Notification>, ServiceError>;
    async fn mark_as_seen(&self, uuid: Uuid) -> Result<bool, ServiceError>;
    async fn delete_notification(&self, uuid: Uuid) -> Result<bool, ServiceError>;
}