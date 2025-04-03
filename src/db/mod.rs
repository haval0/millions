pub mod posts;
pub mod tokens;

use anyhow::Result;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;

pub async fn init_db(host: &str, user: &str) -> Result<Pool<PostgresConnectionManager<NoTls>>> {
    let manager =
        PostgresConnectionManager::new_from_stringlike(format!("host={host} user={user}"), NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    let local = pool.clone();
    let conn = local.get().await?;
    conn.execute(include_str!("../../migrations/0001_init.sql"), &[])
        .await?;
    Ok(pool)
}
