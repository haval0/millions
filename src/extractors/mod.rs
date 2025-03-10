use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(JsonError))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug)]
pub enum JsonError {
    JsonRejection(JsonRejection),
}

impl IntoResponse for JsonError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            JsonError::JsonRejection(rejection) => {
                tracing::error!("JSON parsing failed: {}", rejection.body_text());
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}

impl From<JsonRejection> for JsonError {
    fn from(rejection: JsonRejection) -> Self {
        JsonError::JsonRejection(rejection)
    }
}
