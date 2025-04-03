use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;

use crate::models::post::Post;

pub async fn get_all_posts(db: Client, language: &str) -> Result<Vec<Post>> {
    let posts = db
        .query(
            "SELECT 
p.id,
p.author,
p.publish,
t.language,
COALESCE(t.title, p.title) AS title,
t.description,
e.location,
e.start_time,
e.end_time
FROM 
posts p
LEFT JOIN 
translations t ON p.id = t.post_id AND t.language = $1::varchar
LEFT JOIN 
events e ON p.id = e.post_id
ORDER BY 
p.publish DESC;",
            &[&language],
        )
        .await?
        .iter()
        .map(|row| Post {
            id: row.get("p.id"),
            author: row.get("p.author"),
            publish: row.get("p.publish"),
            language: row.get("t.language"),
            title: row.get("title"),
            description: row.get("t.description"),
            location: row.get("e.location"),
            start_time: row.get("e.start_time"),
            end_time: row.get("e.end_time"),
        })
        .collect();
    Ok(posts)
}
