use crate::{
    AppState,
    errors::http::ServerError,
    extractors::Json,
    models::expo_token::ExpoToken,
    services::calypso::fetch_events,
    templates::{layout, news_feed},
};
use axum::extract::State;
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
