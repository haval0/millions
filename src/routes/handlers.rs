use crate::{
    AppState,
    db::posts::store_post,
    errors::http::ServerError,
    extractors::Json,
    models::{
        expo_token::ExpoToken,
        post::{Event, Post},
    },
    services::calypso::fetch_events,
    templates::{event_form, layout, news_feed, post_form},
};
use axum::{Form, extract::State};
use maud::{Markup, html};

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), ServerError> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)?;
    Ok(())
}

pub async fn news(State(state): State<AppState>) -> Result<Markup, ServerError> {
    let items = fetch_events(&state.client).await?;
    let content = html! {
        section.news {
            h2 { "Millions News Feed" }
            (news_feed(&items))
        }
    };
    Ok(layout("Home", content))
}

pub async fn create_post() -> Result<Markup, ServerError> {
    Ok(layout("Create Post", post_form()))
}

pub async fn submit_post(
    State(state): State<AppState>,
    Form(form): Form<Post>,
) -> Result<(), ServerError> {
    let conn = state.postgres.get_owned().await?;
    tracing::info!(?form, "post form received");
    store_post(conn, form).await?;
    Ok(())
}

pub async fn create_event() -> Result<Markup, ServerError> {
    Ok(layout("Create Event", event_form()))
}

pub async fn submit_event(
    State(state): State<AppState>,
    Form(form): Form<Post>,
) -> Result<(), ServerError> {
    let conn = state.postgres.get_owned().await?;
    tracing::info!(?form, "event form received");
    store_post(conn, form).await?;
    Ok(())
}
