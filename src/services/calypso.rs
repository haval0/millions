use std::cmp::Ordering;

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
    match len.cmp(&25) {
        Ordering::Less => error!(%len, "Calypso yielded fewer than 25 items"),
        Ordering::Greater => warn!(%len, "Calypso yielded more than 25 items"),
        Ordering::Equal => debug!("fetched 25 Calypso items"),
    }
    Ok(response.content)
}
