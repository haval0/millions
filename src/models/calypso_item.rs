use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalypsoItem {
    pub id: i32,
    pub item_type: String,
    pub updated: String,
    pub title_swedish: String,
    pub title_english: String,
    pub author: String,
    pub author_display: String,
    pub publish_as: Option<String>,
    pub publish_as_display: Option<String>,
    pub sticky: bool,
    pub sensitive: bool,
    pub publish_date: String,
    pub content_swedish: String,
    pub content_english: String,
    pub event_location: Option<String>,
    pub event_start_time: Option<String>,
    pub event_end_time: Option<String>,
    pub facebook_event: String,
    pub google_form: String,
    pub publish_status: String,
}
