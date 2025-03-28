use crate::{AppState, templates::not_found};
use axum::{
    Router,
    handler::HandlerWithoutStateExt,
    response::Redirect,
    routing::{get, post},
};
use handlers::news;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub mod handlers;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::to("/news") }))
        .route("/news", get(news))
        .route("/tokens", post(handlers::store_token))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("static").not_found_service(not_found().into_service()))
        .with_state(state)
}
