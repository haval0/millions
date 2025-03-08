use crate::{AppState, models::expo_token::ExpoToken};
use anyhow::Context;
use axum::{Json, extract::State, http::StatusCode};
use tracing::info;

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), axum::http::StatusCode> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)
        .context("Failed to store token")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    info!("Token stored via HTTP: {}", token.token);
    Ok(())
}
