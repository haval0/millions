use time::OffsetDateTime;

#[derive(Debug)]
pub struct Translation {
    pub language: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Event {
    pub location: String,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
}

#[derive(Debug)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub publish: Option<OffsetDateTime>,
    pub translations: Vec<Translation>,
    pub event: Option<Event>,
}
