use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Notification not found")]
    NotFound,

    #[error("Unknown error occurred")]
    Unknown,
}
