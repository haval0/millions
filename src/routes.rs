use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use redb::{Database, ReadableTable, TableDefinition};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, interval};

// Define the tokens table in redb
pub const TOKENS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("tokens");

// Expo token payload
#[derive(Deserialize, Serialize)]
pub struct ExpoToken {
    pub token: String,
}

// App state (shared across handlers)
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub client: Client,
}

// Handler to store a token
pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), String> {
    let write_txn = state.db.begin_write().map_err(|e| e.to_string())?;
    {
        let mut table = write_txn
            .open_table(TOKENS_TABLE)
            .map_err(|e| e.to_string())?;
        table
            .insert(token.token.as_str(), "active")
            .map_err(|e| e.to_string())?;
    }
    write_txn.commit().map_err(|e| e.to_string())?;
    Ok(())
}

// Background task to poll and notify
pub async fn poll_and_notify(state: AppState) {
    let mut interval = interval(Duration::from_secs(10));
    let mut last_state: Option<String> = None;

    loop {
        interval.tick().await;
        let response = state
            .client
            .get("https://calypso.datasektionen.se/api/list")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if last_state.as_ref() != Some(&response) {
            notify_all(&state).await;
            last_state = Some(response);
        }
    }
}

pub async fn notify_all(state: &AppState) {
    let read_txn = state.db.begin_read().unwrap();
    let table = read_txn.open_table(TOKENS_TABLE).unwrap();
    for token in table.iter().unwrap() {
        let token = token.unwrap();
        let token_str = token.0.value();
        state
            .client
            .post("https://exp.host/--/api/v2/push/send")
            .json(&serde_json::json!({
                "to": token_str,
                "title": "Update",
                "body": "Something changed!"
            }))
            .send()
            .await
            .unwrap();
    }
}

// Setup the Axum router
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/tokens", post(store_token))
        .with_state(state)
}
