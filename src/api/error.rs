use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[allow(dead_code)]
pub struct ApiError {
    code: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(code: StatusCode, message: String) -> Self {
        ApiError { code, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        error!("{:?}", err.into());
        ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
        }
    }
}
