use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub language: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub location: String,
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end_time: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    // Don't set id when sending to db
    pub id: Option<i32>,
    pub author: String,
    pub title: String,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub publish: Option<OffsetDateTime>,
    pub translations: Vec<Translation>,
    #[serde(default)]
    pub event: Option<Event>,
}
