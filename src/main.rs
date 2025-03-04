use std::sync::Arc;

use redb::{Database, WriteTransaction};
use reqwest::Client;
use routes::{AppState, TOKENS_TABLE, create_app};

mod routes;

#[tokio::main]
async fn main() {
    let db = Arc::new(Database::create("tokens.redb").unwrap());

    {
        let write_txn = db.begin_write().unwrap();
        initialize_tables(&write_txn);
        write_txn.commit().unwrap();
    }

    let client = Client::new();

    let state = AppState {
        db: db.clone(),
        client: client.clone(),
    };

    tokio::spawn(routes::poll_and_notify(state.clone()));

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn initialize_tables(txn: &WriteTransaction) {
    // Create tables if they don’t exist
    txn.open_table(TOKENS_TABLE)
        .unwrap_or_else(|_| txn.open_table(TOKENS_TABLE).unwrap());
}
