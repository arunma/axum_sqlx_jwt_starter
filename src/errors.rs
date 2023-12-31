use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::errors::ApiError::{
    BadRequest, InternalServerErrorWithContext, InvalidLoginAttempt, NotFound, ObjectConflict, Unauthorized,
};

pub type ApiResult<T> = Result<T, ApiError>;

//pub type ErrorMap = HashMap<String, Vec<String>>;

//TODO - Cleanup
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("username or password is incorrect")]
    InvalidLoginAttempt,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ApplicationStartup(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithContext(String),
    #[error("{0}")]
    ObjectConflict(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Unauthorized => (StatusCode::UNAUTHORIZED, Unauthorized.to_string()),
            InvalidLoginAttempt => (StatusCode::BAD_REQUEST, InvalidLoginAttempt.to_string()),
            NotFound(e) => (StatusCode::NOT_FOUND, e),
            BadRequest(e) => (StatusCode::BAD_REQUEST, e),
            InternalServerErrorWithContext(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            ObjectConflict(e) => (StatusCode::CONFLICT, e),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error".to_string()),
        };

        (status, error_message).into_response()
    }
}
