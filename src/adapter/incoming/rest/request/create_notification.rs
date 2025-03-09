use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub(crate) message: String,
}