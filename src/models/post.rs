use time::OffsetDateTime;

#[derive(Debug)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub publish: Option<OffsetDateTime>,
    pub language: String,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub start_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
}
