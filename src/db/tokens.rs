use anyhow::Result;
use redb::{Database, ReadableTable, TableDefinition};
use tracing::info;

pub const TOKENS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("tokens");

pub fn init_database(path: &str) -> Result<Database> {
    let db = Database::create(path)?;
    let write_txn = db.begin_write()?;
    {
        write_txn.open_table(TOKENS_TABLE)?;
    }
    write_txn.commit()?;
    info!("Database initialized at {}", path);
    Ok(db)
}

pub fn store_token(db: &Database, token: &str) -> Result<()> {
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TOKENS_TABLE)?;
        table.insert(token, "")?;
    }
    write_txn.commit()?;
    info!("Stored token: {}", token);
    Ok(())
}

pub fn get_all_tokens(db: &Database) -> Result<Vec<String>> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TOKENS_TABLE)?;
    let tokens = table
        .iter()?
        .map(|result| result.map(|(token, _)| token.value().to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    info!("Retrieved {} tokens", tokens.len());
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_get_tokens() -> Result<()> {
        let temp_path = "test_tokens.redb";
        let db = init_database(temp_path)?;

        store_token(&db, "token1")?;
        store_token(&db, "token2")?;

        let tokens = get_all_tokens(&db)?;
        assert_eq!(tokens.len(), 2);
        assert!(tokens.contains(&"token1".to_string()));
        assert!(tokens.contains(&"token2".to_string()));

        std::fs::remove_file(temp_path)?;
        Ok(())
    }
}
