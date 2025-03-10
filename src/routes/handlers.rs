use crate::{AppState, errors::AppError, extractors::Json, models::expo_token::ExpoToken};
use axum::extract::State;
use tracing::info;

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), AppError> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)?;
    info!("Token stored via HTTP: {}", token.token);
    Ok(())
}
