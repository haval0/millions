use crate::AppState;
use axum::{Router, routing::post};
use tower_http::trace::TraceLayer;

pub mod handlers;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/tokens", post(handlers::store_token))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
