use crate::{AppState, db::tokens, models::calypso_item::CalypsoItem, services::calypso};
use serde_json::json;
use tokio::time::{Duration, interval};
use tracing::{error, info};

pub async fn start_polling(state: AppState) {
    let mut interval = interval(Duration::from_secs(10));
    let mut last_items: Option<Vec<CalypsoItem>> = None;

    loop {
        interval.tick().await;

        match calypso::fetch_events(&state.client).await {
            Ok(current_items) => {
                if let Some(ref prev) = last_items {
                    if let Some(new_item) = detect_new_item(prev, &current_items) {
                        info!("Detected new item: {:?}", new_item);
                        notify_all(&state, &new_item).await;
                    }
                }
                last_items = Some(current_items);
            }
            Err(e) => error!("Failed to fetch Calypso events: {:?}", e),
        }
    }
}

pub fn detect_new_item(prev: &[CalypsoItem], current: &[CalypsoItem]) -> Option<CalypsoItem> {
    let current_top = &current[0];
    if current_top.id != prev.first().map_or(0, |item| item.id)
        && !prev.iter().any(|item| item.id == current_top.id)
    {
        Some(current_top.clone())
    } else {
        None
    }
}

pub async fn notify_all(state: &AppState, new_item: &CalypsoItem) {
    match tokens::get_all_tokens(&state.db) {
        Ok(tokens) => {
            for token in tokens {
                let result = state.client
                    .post("https://exp.host/--/api/v2/push/send")
                    .json(&json!({
                        "to": token,
                        "title": match new_item.item_type.as_str() {
                            "EVENT" => "New Event",
                            "POST" => "New Post",
                            _ => "New Update",
                        },
                        "body": format!(
                            "{}: {}",
                            if new_item.title_english.is_empty() { &new_item.title_swedish } else { &new_item.title_english },
                            new_item.content_english.chars().take(100).collect::<String>()
                        )
                    }))
                    .send()
                    .await;
                match result {
                    Ok(_) => info!(
                        "Sent notification for item {} to token {}",
                        new_item.id, token
                    ),
                    Err(e) => error!("Failed to send notification to {}: {:?}", token, e),
                }
            }
        }
        Err(e) => error!("Failed to get tokens: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_new_item() {
        let prev = vec![
            CalypsoItem {
                id: 11715,
                item_type: "POST".to_string(),
                title_swedish: "Tech-i-taka pub".to_string(),
                title_english: "Tech-i-taka pub".to_string(),
                updated: "2025-03-03T13:42:39.201566".to_string(),
                author: "lydiabr".to_string(),
                author_display: "Lydia Brorsson".to_string(),
                publish_as: None,
                publish_as_display: None,
                sticky: false,
                sensitive: false,
                publish_date: "2025-03-03T13:42:39.1936".to_string(),
                content_swedish: "Om ni har missat...".to_string(),
                content_english: "If you have missed...".to_string(),
                event_location: None,
                event_start_time: None,
                event_end_time: None,
                facebook_event: "https://fb.me/e/4mc5u0hES".to_string(),
                google_form: "https://dsekt.se/tit".to_string(),
                publish_status: "PUBLISHED".to_string(),
            },
            // Simulate 24 more items
        ];

        let current = vec![
            CalypsoItem {
                id: 11716, // New item
                item_type: "EVENT".to_string(),
                title_swedish: "New Event".to_string(),
                title_english: "New Event".to_string(),
                updated: "2025-03-04T10:00:00".to_string(),
                author: "test".to_string(),
                author_display: "Test User".to_string(),
                publish_as: None,
                publish_as_display: None,
                sticky: false,
                sensitive: false,
                publish_date: "2025-03-04T10:00:00".to_string(),
                content_swedish: "Nytt event...".to_string(),
                content_english: "New event...".to_string(),
                event_location: Some("META".to_string()),
                event_start_time: Some("2025-03-05T14:00:00".to_string()),
                event_end_time: Some("2025-03-05T16:00:00".to_string()),
                facebook_event: "".to_string(),
                google_form: "".to_string(),
                publish_status: "PUBLISHED".to_string(),
            },
            CalypsoItem {
                id: 11715,
                ..prev[0].clone()
            },
            // Simulate 23 more items
        ];

        let new_item = detect_new_item(&prev, &current);
        assert!(new_item.is_some());
        assert_eq!(new_item.unwrap().id, 11716);

        let same = vec![
            CalypsoItem {
                id: 11715,
                ..prev[0].clone()
            },
            // Same as prev
        ];
        let no_new = detect_new_item(&prev, &same);
        assert!(no_new.is_none());
    }
}
