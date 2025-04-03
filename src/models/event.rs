use time::OffsetDateTime;

#[derive(Debug)]
pub struct Event {
    pub id: i32,
    pub author: String,
    pub publish: OffsetDateTime,
    pub language: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
}
