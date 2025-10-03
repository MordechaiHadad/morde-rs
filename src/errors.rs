#[cfg(feature = "axum")]
use axum::{
    Json,
    response::{IntoResponse, Response},
};

use http::StatusCode;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Extensible status kind: either a real HTTP status or an application numeric status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusKind {
    Http(StatusCode),
    App(u16),
}

impl StatusKind {
    /// Map to an HTTP status for transport adapters. Application numeric statuses
    /// default to 500 (Internal Server Error) unless adapters provide a different mapping.
    pub fn to_http_status(&self) -> StatusCode {
        match self {
            StatusKind::Http(c) => *c,
            StatusKind::App(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<StatusCode> for StatusKind {
    fn from(c: StatusCode) -> Self {
        StatusKind::Http(c)
    }
}

impl From<u16> for StatusKind {
    fn from(n: u16) -> Self {
        StatusKind::App(n)
    }
}

#[derive(Debug)]
pub struct AppError {
    pub status: StatusKind,
    pub error: String,
    pub message: String,
}

impl AppError {
    pub fn new(status: StatusKind, error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            error: error.into(),
            message: message.into(),
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::new(
            StatusKind::Http(StatusCode::INTERNAL_SERVER_ERROR),
            "INTERNAL_SERVER_ERROR",
            message,
        )
    }

    pub fn not_found(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusKind::Http(StatusCode::NOT_FOUND), error, message)
    }

    pub fn bad_request(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusKind::Http(StatusCode::BAD_REQUEST), error, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusKind::Http(StatusCode::UNAUTHORIZED), "UNAUTHORIZED", message)
    }

    pub fn conflict(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusKind::Http(StatusCode::CONFLICT), error, message)
    }

    // pub fn forbidden(message: impl Into<String>) -> Self {
    //     Self::new(StatusCode::FORBIDDEN, "FORBIDDEN", message)
    // }
}

/// Convenient crate-local result alias.
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(feature = "axum")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            error: self.error,
            message: self.message,
        };

        (self.status.to_http_status(), Json(error_response)).into_response()
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::internal_server_error(err.to_string())
    }
}

#[cfg(feature = "surrealdb")]
impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        Self::internal_server_error(format!("Database error: {err}"))
    }
}

#[cfg(feature = "eyre")]
impl From<eyre::Report> for AppError {
    fn from(err: eyre::Report) -> Self {
        Self::internal_server_error(err.to_string())
    }
}
#[macro_export]
macro_rules! app_error {
    (custom($n:expr), $error:expr, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("App Status {}: {} - {}: {}", $n, $error, $msg, $internal_msg);
        $crate::types::errors::AppError::new(
            $crate::types::errors::StatusKind::App($n),
            $error,
            $msg,
        )
    }};

    (http($status:expr), $error:expr, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("HTTP {}: {} - {}: {}", $status, $error, $msg, $internal_msg);
        $crate::types::errors::AppError::new(
            $crate::types::errors::StatusKind::Http($status),
            $error,
            $msg,
        )
    }};

    // Matches AppError::bad_request
    (bad_request, $error:expr, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("Bad Request: {} - {}: {}", $error, $msg, $internal_msg);
        $crate::types::errors::AppError::bad_request($error, $msg)
    }};

    // Matches AppError::not_found
    (not_found, $error:expr, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("Not Found: {} - {}: {}", $error, $msg, $internal_msg);
        $crate::types::errors::AppError::not_found($error, $msg)
    }};

    // Matches AppError::unauthorized
    (unauthorized, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("Unauthorized: {}: {}", $msg, $internal_msg);
        $crate::types::errors::AppError::unauthorized($msg)
    }};

    // Matches AppError::conflict
    (conflict, $error:expr, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::warn!("Conflict: {} - {}: {}", $error, $msg, $internal_msg);
        $crate::types::errors::AppError::conflict($error, $msg)
    }};

    // Matches AppError::internal_server_error
    (internal_server_error, $msg:expr $(, $internal_msg:expr)?) => {{
        tracing::error!("Internal Server Error: {}: {}", $msg, $internal_msg);
        $crate::types::errors::AppError::internal_server_error($msg)
    }};
}
