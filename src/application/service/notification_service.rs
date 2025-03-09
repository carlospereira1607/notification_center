use crate::application::error::ApplicationError;
use crate::domain::model::notification::Notification;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait NotificationServicePort: Send + Sync {
    async fn create_notification(&self, message: String) -> Result<Notification, ApplicationError>;
    async fn get_notification(&self, uuid: String) -> Result<Notification, ApplicationError>;
    async fn list_notifications(&self) -> Result<Vec<Notification>, ApplicationError>;
    async fn mark_as_seen(&self, uuid: String) -> Result<bool, ApplicationError>;
    async fn mark_as_deleted(&self, uuid: String) -> Result<bool, ApplicationError>;
}