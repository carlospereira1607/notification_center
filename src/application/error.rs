use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;


#[derive(Debug)]
pub enum ApplicationError {
    NotFound(String),
    InternalError(String),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            ApplicationError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": message })),
            )
                .into_response(),

            ApplicationError::InternalError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": message })),
            )
                .into_response(),
        }
    }
}
