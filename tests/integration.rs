use millions::{AppState, db, services::notifications};
use reqwest::Client;
use std::sync::Arc;
use tokio;

#[tokio::test]
async fn test_store_token_and_notify() {
    let db = Arc::new(db::tokens::init_database("test_integration.redb").unwrap());
    let client = Client::new();
    let state = AppState {
        db: db.clone(),
        client,
    };

    db::tokens::store_token(&db, "test_token").unwrap();

    let handle = tokio::spawn(notifications::start_polling(state.clone()));
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    handle.abort();

    std::fs::remove_file("test_integration.redb").unwrap();
}
