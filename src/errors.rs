use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct HandlerError(anyhow::Error);

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        tracing::error!("Handler error: {:?}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for HandlerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
