use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    SpotifyError(String),
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::SpotifyError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };
        (status, axum::Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnvVar(String),
    DatabaseError(sqlx::Error),
    MigrateError(sqlx::migrate::MigrateError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEnvVar(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
            ConfigError::DatabaseError(e) => write!(f, "Database connection failed: {}", e),
            ConfigError::MigrateError(e) => write!(f, "Migration failed: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<sqlx::Error> for ConfigError {
    fn from(err: sqlx::Error) -> Self {
        ConfigError::DatabaseError(err)
    }
}

impl From<sqlx::migrate::MigrateError> for ConfigError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        ConfigError::MigrateError(err)
    }
}
