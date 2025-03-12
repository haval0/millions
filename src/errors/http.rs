use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct ClientError(anyhow::Error);

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        tracing::error!("Client error: {:?}", self.0);
        StatusCode::BAD_REQUEST.into_response()
    }
}

impl<E> From<E> for ClientError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Debug)]
pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        tracing::error!("Server error: {:?}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
