use crate::adapter::outgoing::persistence::entity::notification_entity::NotificationEntity;
use crate::application::error::ApplicationError;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait NotificationRepositoryPort {
    async fn save(&self, notification: &NotificationEntity) -> Result<NotificationEntity, ApplicationError>;

    async fn get(&self, uuid: &Uuid) -> Result<NotificationEntity, ApplicationError>;

    async fn get_all(&self) -> Result<Vec<NotificationEntity>, ApplicationError>;
}