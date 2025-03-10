use crate::models::calypso_item::CalypsoItem;
use anyhow::Result;
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
        .await?
        .json::<CalypsoResponse>()
        .await?;

    info!("Fetched {} Calypso items", response.content.len());
    Ok(response.content)
}
