use crate::{AppState, errors::HandlerError, extractors::Json, models::expo_token::ExpoToken};
use anyhow::Context;
use axum::extract::State;
use tracing::info;

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), HandlerError> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)
        .context("Failed to store token")?;
    info!("Token stored via HTTP: {}", token.token);
    Ok(())
}
