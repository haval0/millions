use millions::{AppState, db, routes::create_app, services::start_polling};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting application");

    let db = Arc::new(db::tokens::init_database("tokens.redb")?);
    let client = reqwest::Client::new();

    let state = AppState { db, client };

    info!("Spawning Calypso polling task");
    tokio::spawn(start_polling(state.clone()));

    let app = create_app(state);
    info!("Starting server on 0.0.0.0:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
