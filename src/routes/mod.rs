use crate::{AppState, templates::not_found};
use axum::{
    Router,
    handler::HandlerWithoutStateExt,
    response::Redirect,
    routing::{get, post},
};
use handlers::{create_event, create_post, news, store_token, submit_event, submit_post};
use tower_http::{services::ServeDir, trace::TraceLayer};

pub mod handlers;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::to("/news") }))
        .route("/news", get(news))
        .route("/tokens", post(store_token))
        .route("/posts/create", get(create_post).post(submit_post))
        .route("/events/create", get(create_event).post(submit_event))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("static").not_found_service(not_found().into_service()))
        .with_state(state)
}
