use crate::AppState;
use axum::{Router, routing::post};

pub mod handlers;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/tokens", post(handlers::store_token))
        .with_state(state)
}
