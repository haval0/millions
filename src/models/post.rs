use time::OffsetDateTime;

#[derive(Debug)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub publish: OffsetDateTime,
    pub language: String,
    pub title: String,
    pub description: String,
}
