use crate::models::calypso_item::CalypsoItem;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, error, warn};

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

    let len = response.content.len();
    if len < 25 {
        error!(%len, "Calypso yielded fewer than 25 items");
    } else if len == 25 {
        debug!("fetched 25 Calypso items");
    } else {
        warn!(%len, "Calypso yielded more than 25 items");
    }
    Ok(response.content)
}
