use crate::adapter::incoming::rest::notification_api::NotificationApi;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub notification_api: Arc<NotificationApi>,
}