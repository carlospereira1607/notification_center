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