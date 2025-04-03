use millions::{AppState, db, routes::create_app, services::start_polling};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "millions=info,tower_http=debug,axum::rejection=trace".into()),
        )
        .init();

    info!("Starting application");

    let db = Arc::new(db::tokens::init_database("tokens.redb")?);
    let postgres = db::init_db("0.0.0.0", "millions", "millions").await?;
    let client = reqwest::Client::new();

    let state = AppState {
        db,
        postgres,
        client,
    };

    info!("Spawning Calypso polling task");
    tokio::spawn(start_polling(state.clone()));

    let app = create_app(state);
    info!("Starting server on 0.0.0.0:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
