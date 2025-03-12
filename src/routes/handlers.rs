use crate::{AppState, errors::http::ServerError, extractors::Json, models::expo_token::ExpoToken};
use axum::extract::State;

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), ServerError> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)?;
    Ok(())
}
