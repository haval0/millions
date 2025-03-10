use redb::Database;
use reqwest::Client;
use std::sync::Arc;

pub mod db;
pub mod errors;
pub mod extractors;
pub mod models;
pub mod routes;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub client: Client,
}
