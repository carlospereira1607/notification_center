use crate::adapter::incoming::rest::request::create_notification::CreateNotificationRequest;
use crate::adapter::incoming::rest::response::notification::NotificationResponse;
use crate::application::configuration::application_state::AppState;
use crate::application::error::ApplicationError;
use crate::application::service::notification_service::NotificationServicePort;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub struct NotificationApi {
    notification_service: Arc<dyn NotificationServicePort + Send + Sync>,
}

impl NotificationApi {
    pub fn new(notification_service: Arc<dyn NotificationServicePort + Send + Sync>) -> Self {
        Self { notification_service }
    }
}

pub async fn create_notification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNotificationRequest>,
) -> Result<Json<NotificationResponse>, ApplicationError> {
    let notification = state.notification_api.notification_service.create_notification(payload.message).await?;
    Ok(Json(notification.into()))
}

pub async fn get_notification(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<Json<NotificationResponse>, ApplicationError> {
    let notification = state.notification_api.notification_service.get_notification(uuid).await?;
    Ok(Json(notification.into()))
}

pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<NotificationResponse>>, ApplicationError> {
    let notifications = state.notification_api.notification_service.list_notifications().await?;
    let notifications_response: Vec<NotificationResponse> = notifications.iter()
        .map(move |notification| { notification.clone().into() }).collect();

    Ok(Json(notifications_response.into()))
}

pub async fn mark_as_seen(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<Json<bool>, ApplicationError> {
    let result = state.notification_api.notification_service.mark_as_seen(uuid).await?;
    Ok(Json(result))
}

pub async fn mark_as_deleted(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<Json<bool>, ApplicationError> {
    let result = state.notification_api.notification_service.mark_as_deleted(uuid).await?;
    Ok(Json(result))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/notification", post(create_notification).get(get_notifications))
        .route("/notification/{uuid}", get(get_notification))
        .route("/notification/seen/{uuid}", post(mark_as_seen))
        .route("/notification/deleted/{uuid}", post(mark_as_deleted))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::incoming::rest::notification_api::NotificationApi;
    use crate::adapter::incoming::rest::request::create_notification::CreateNotificationRequest;
    use crate::application::configuration::application_state::AppState;
    use crate::application::service::notification_service::MockNotificationServicePort;
    use crate::domain::model::notification::Notification;
    use axum::extract::State;
    use axum::Json;
    use mockall::predicate::eq;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_notification_success() {
        let mut mock_service = MockNotificationServicePort::new();
        let notification = Notification::new("test message".to_string());
        let request = CreateNotificationRequest {
            message: "test message".to_string(),
        };

        mock_service
            .expect_create_notification()
            .with(eq("test message".to_string()))
            .returning(move |_| Ok(notification.clone()));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = create_notification(State(app_state), Json(request)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.message, "test message");
    }

    #[tokio::test]
    async fn test_get_notification_success() {
        let mut mock_service = MockNotificationServicePort::new();
        let uuid = Uuid::new_v4().to_string();
        let notification = Notification::new("existing message".to_string());

        mock_service
            .expect_get_notification()
            .with(eq(uuid.clone()))
            .returning(move |_| Ok(notification.clone()));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = get_notification(State(app_state), Path(uuid)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.message, "existing message");
    }

    #[tokio::test]
    async fn test_get_notifications_success() {
        let mut mock_service = MockNotificationServicePort::new();
        mock_service
            .expect_list_notifications()
            .returning(move || Ok(
                vec![
                    Notification::new("message 1".to_string()),
                    Notification::new("message 2".to_string()),
                ]
            ));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = get_notifications(State(app_state)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].message, "message 1");
        assert_eq!(response[1].message, "message 2");
    }

    #[tokio::test]
    async fn test_get_notifications_empty_success() {
        let mut mock_service = MockNotificationServicePort::new();
        mock_service
            .expect_list_notifications()
            .returning(move || Ok(vec![]));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = get_notifications(State(app_state)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 0);
    }

    #[tokio::test]
    async fn test_mark_as_seen_success() {
        let mut mock_service = MockNotificationServicePort::new();
        let uuid = Uuid::new_v4().to_string();

        mock_service
            .expect_mark_as_seen()
            .with(eq(uuid.clone()))
            .returning(move |_| Ok(true));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = mark_as_seen(State(app_state), Path(uuid)).await;

        assert!(result.is_ok());
        assert!(result.unwrap().0);
    }

    #[tokio::test]
    async fn test_mark_as_deleted_success() {
        let mut mock_service = MockNotificationServicePort::new();
        let uuid = Uuid::new_v4().to_string();

        mock_service
            .expect_mark_as_deleted()
            .with(eq(uuid.clone()))
            .returning(move |_| Ok(true));

        let app_state = Arc::new(AppState {
            notification_api: Arc::new(NotificationApi::new(Arc::new(mock_service))),
        });

        let result = mark_as_deleted(State(app_state), Path(uuid)).await;

        assert!(result.is_ok());
        assert!(result.unwrap().0);
    }
}