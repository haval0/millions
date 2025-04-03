use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;

use crate::models::post::{Event, Post, Translation};

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
            translations t ON p.id = t.post_id AND t.language = $1
            LEFT JOIN
            events e ON p.id = e.post_id
            ORDER BY
            p.publish DESC;",
            &[&language],
        )
        .await?
        .iter()
        .map(|row| {
            let translations = vec![Translation {
                language: language.to_owned(),
                title: row.get("title"),
                description: row.get("t.description"),
            }];
            let location: Option<String> = row.get("e.location");
            let event = if location.is_some() {
                Some(Event {
                    location: row.get("e.location"),
                    start_time: row.get("e.start_time"),
                    end_time: row.get("e.end_time"),
                })
            } else {
                None
            };
            Post {
                id: row.get("p.id"),
                author: row.get("p.author"),
                title: row.get("title"),
                publish: row.get("p.publish"),
                translations,
                event,
            }
        })
        .collect();
    Ok(posts)
}

pub async fn store_post(mut db: Client, post: Post) -> Result<()> {
    let transaction = db.transaction().await?;
    let post_row = transaction
        .query_one(
            "INSERT INTO posts (author, title, publish)
        VALUES ($1, $2, $3)",
            &[&post.author, &post.title, &post.publish],
        )
        .await?;

    let post_id: i32 = post_row.get("id");

    for translation in post.translations {
        transaction
            .execute(
                "INSERT INTO translations (post_id, language, title, description)
                VALUES ($1, $2, $3, $4)",
                &[
                    &post_id,
                    &translation.language,
                    &translation.title,
                    &translation.description,
                ],
            )
            .await?;
    }

    if let Some(event) = post.event {
        transaction
            .execute(
                "INSERT INTO events (post_id, location, start_time, end_time)
            VALUES ($1, $2, $3, $4)",
                &[
                    &post_id,
                    &event.location,
                    &event.start_time,
                    &event.end_time,
                ],
            )
            .await?;
    }
    transaction.commit().await?;
    Ok(())
}
