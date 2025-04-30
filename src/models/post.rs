use serde::{Deserialize, Deserializer, de};
use time::{OffsetDateTime, format_description::well_known::Iso8601};

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub language: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub location: String,
    #[serde(deserialize_with = "deserialize_time")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_time: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    // Don't set id for new posts
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_option_time")]
    pub publish: Option<OffsetDateTime>,
    #[serde(default)]
    pub translations: Vec<Translation>,
    #[serde(default)]
    pub event: Option<Event>,
}

fn deserialize_time<'de, D>(de: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let str = <String>::deserialize(de)?;
    OffsetDateTime::parse(&str, &Iso8601::DATE_TIME).map_err(de::Error::custom)
}

fn deserialize_option_time<'de, D>(de: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => OffsetDateTime::parse(&s, &Iso8601::DATE_TIME)
            .map_err(de::Error::custom)
            .map(Some),
    }
}
