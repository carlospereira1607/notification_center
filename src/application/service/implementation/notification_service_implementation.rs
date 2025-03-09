use crate::adapter::outgoing::persistence::entity::notification_entity::NotificationEntity;
use crate::application::error::ApplicationError;
use crate::application::service::notification_service::NotificationServicePort;
use crate::domain::model::notification::Notification;
use crate::port::outgoing::persistence::notification_repository_port::NotificationRepositoryPort;
use async_trait::async_trait;
use uuid::Uuid;

pub struct NotificationServiceImplementation<R: NotificationRepositoryPort + Sync + Send> {
    notification_repository: R,
}

impl<R: NotificationRepositoryPort + Sync + Send> NotificationServiceImplementation<R> {
    pub fn new(repository: R) -> Self {
        Self { notification_repository: repository }
    }
}

#[async_trait]
impl<R: NotificationRepositoryPort + Sync + Send> NotificationServicePort for NotificationServiceImplementation<R> {
    async fn create_notification(&self, message: String) -> Result<Notification, ApplicationError> {
        let notification = Notification::new(message);
        let notification_entity: NotificationEntity = notification.clone().into();
        self.notification_repository.save(&notification_entity).await?;
        Ok(notification)
    }

    async fn get_notification(&self, uuid: String) -> Result<Notification, ApplicationError> {
        let parsed_uuid = Uuid::parse_str(uuid.as_str()).unwrap();
        let notification_entity = self.notification_repository.get(&parsed_uuid).await?;
        Ok(notification_entity.into())
    }

    async fn list_notifications(&self) -> Result<Vec<Notification>, ApplicationError> {
        let notification_entities = self.notification_repository.get_all().await?;
        let notifications = notification_entities.into_iter().map(|entity| entity.into()).collect();
        Ok(notifications)
    }

    async fn mark_as_seen(&self, uuid: String) -> Result<bool, ApplicationError> {
        let parsed_uuid = Uuid::parse_str(uuid.as_str()).unwrap();
        let notification_entity = self.notification_repository.get(&parsed_uuid).await?;
        let mut notification: Notification = notification_entity.into();
        notification.set_as_seen();
        let updated_entity: NotificationEntity = notification.into();
        Ok(self.notification_repository.save(&updated_entity).await?.seen)
    }

    async fn mark_as_deleted(&self, uuid: String) -> Result<bool, ApplicationError> {
        let parsed_uuid = Uuid::parse_str(uuid.as_str()).unwrap();
        let notification_entity = self.notification_repository.get(&parsed_uuid).await?;
        let mut notification: Notification = notification_entity.into();
        notification.set_as_deleted();
        let updated_entity: NotificationEntity = notification.into();
        Ok(self.notification_repository.save(&updated_entity).await?.deleted)
    }
}

//TODO pending to add error cases
#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::outgoing::persistence::notification_repository_port::MockNotificationRepositoryPort;
    use tokio;

    #[tokio::test]
    async fn test_create_notification_success() {
        let notification = generate_random_notification();
        let message = notification.uuid.clone().to_string();

        let mut mocked_repo = MockNotificationRepositoryPort::new();
        mocked_repo.expect_save()
            .times(1)
            .returning(|notification_entity| Ok(notification_entity.clone()));

        let notification_service = NotificationServiceImplementation::new(mocked_repo);

        let result = notification_service.create_notification(message.clone()).await;

        let result = result.unwrap();

        assert_eq!(result.message, message);
        assert_eq!(result.seen, false);
        assert_eq!(result.deleted, false);
    }

    #[tokio::test]
    async fn test_get_notification_success() {
        let mut notification = generate_random_notification();
        notification.seen = true;

        let message = notification.message.clone().to_string();
        let uuid_to_look_for = notification.uuid.clone();
        let uuid_to_look_for_as_string = notification.uuid.to_string().clone();

        let mut mocked_repo = MockNotificationRepositoryPort::new();
        mocked_repo.expect_get()
            .times(1)
            .returning(move |_| Ok(notification.clone().into()));

        let notification_service = NotificationServiceImplementation::new(mocked_repo);

        let result = notification_service.get_notification(uuid_to_look_for_as_string.clone()).await;

        let result = result.unwrap();

        assert_eq!(result.message, message);
        assert!(result.seen);
        assert_eq!(result.deleted, false);
        assert_eq!(result.uuid, uuid_to_look_for);
    }

    #[tokio::test]
    async fn test_list_notifications_success() {
        let mut mocked_repo = MockNotificationRepositoryPort::new();
        mocked_repo.expect_get_all()
            .times(1)
            .returning(|| {
                let mut notifications: Vec<NotificationEntity> = Vec::new();
                for _ in 0..10 {
                    let notification_entity: NotificationEntity = generate_random_notification().into();
                    notifications.push(notification_entity);
                }
                Ok(notifications)
            });

        let notification_service = NotificationServiceImplementation::new(mocked_repo);

        let result = notification_service.list_notifications().await.unwrap();

        assert_eq!(result.len(), 10);
    }

    #[tokio::test]
    async fn test_list_notifications_with_empty_result_success() {
        let mut mocked_repo = MockNotificationRepositoryPort::new();
        mocked_repo.expect_get_all()
            .times(1)
            .returning(|| Ok(vec![]));

        let notification_service = NotificationServiceImplementation::new(mocked_repo);

        let result = notification_service.list_notifications().await.unwrap();

        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_mark_as_seen_success() {
        let notification = generate_random_notification();
        let original_notification_entity: NotificationEntity = notification.clone().into();

        let updated_entity = NotificationEntity {
            seen: true,
            last_updated: chrono::Utc::now(),
            ..original_notification_entity.clone()
        };

        let mut mock_repo = MockNotificationRepositoryPort::new();
        mock_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(original_notification_entity.clone()));

        mock_repo
            .expect_save()
            .times(1)
            .returning(move |_| Ok(updated_entity.clone()));

        let service = NotificationServiceImplementation::new(mock_repo);

        let uuid_to_search_for = notification.uuid.to_string();
        let result = service.mark_as_seen(uuid_to_search_for).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mark_as_deleted_success() {
        let notification = generate_random_notification();
        let original_notification_entity: NotificationEntity = notification.clone().into();

        let updated_entity = NotificationEntity {
            seen: true,
            deleted: true,
            last_updated: chrono::Utc::now(),
            ..original_notification_entity.clone()
        };

        let mut mock_repo = MockNotificationRepositoryPort::new();
        mock_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(original_notification_entity.clone()));

        mock_repo
            .expect_save()
            .times(1)
            .returning(move |_| Ok(updated_entity.clone()));

        let service = NotificationServiceImplementation::new(mock_repo);

        let uuid_to_search_for = notification.uuid.to_string();
        let result = service.mark_as_deleted(uuid_to_search_for).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }


    fn generate_random_notification() -> Notification {
        let random_message = Uuid::new_v4().to_string();
        Notification::new(random_message)
    }
}
