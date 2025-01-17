use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[allow(dead_code)]
pub struct UiError {
    code: StatusCode,
    message: String,
}

impl UiError {
    pub fn new(code: StatusCode, message: String) -> Self {
        UiError { code, message }
    }
}

impl IntoResponse for UiError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

impl<E> From<E> for UiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        error!("{:?}", err.into());
        UiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
        }
    }
}
