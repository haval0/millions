use bb8_postgres::{PostgresConnectionManager, bb8::Pool, tokio_postgres::NoTls};
use redb::Database;
use reqwest::Client;
use std::sync::Arc;

pub mod db;
pub mod errors;
pub mod extractors;
pub mod models;
pub mod routes;
pub mod services;
pub mod templates;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub postgres: Pool<PostgresConnectionManager<NoTls>>,
    pub client: Client,
}
