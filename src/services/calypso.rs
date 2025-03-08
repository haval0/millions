use crate::models::calypso_item::CalypsoItem;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct CalypsoResponse {
    content: Vec<CalypsoItem>,
}

pub async fn fetch_events(client: &Client) -> Result<Vec<CalypsoItem>> {
    let response = client
        .get("https://calypso.datasektionen.se/api/list")
        .send()
        .await
        .context("Failed to fetch Calypso events")?;

    let calypso_response = response
        .json::<CalypsoResponse>()
        .await
        .context("Failed to parse Calypso response")?;

    info!("Fetched {} Calypso items", calypso_response.content.len());
    Ok(calypso_response.content)
}
