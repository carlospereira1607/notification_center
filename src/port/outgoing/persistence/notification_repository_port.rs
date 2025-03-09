use crate::adapter::outgoing::persistence::entity::notification_entity::NotificationEntity;
use crate::application::error::ServiceError;
use async_trait::async_trait;
use mockall::automock;
use std::future::Future;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait NotificationRepositoryPort {
    async fn save(&self, notification: &NotificationEntity) -> Result<NotificationEntity, ServiceError>;

    async fn get(&self, uuid: &Uuid) -> Result<NotificationEntity, ServiceError>;

    async fn get_all(&self) -> Result<Vec<NotificationEntity>, ServiceError>;
}