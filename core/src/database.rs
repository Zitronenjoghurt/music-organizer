use redb::TableDefinition;
use std::path::Path;

pub mod entry;
pub mod file;
pub mod info;

const SONGS: TableDefinition<u64, Vec<u8>> = TableDefinition::new("songs");

pub struct Database {
    db: redb::Database,
}

impl Database {
    pub fn attach(path: impl AsRef<Path>) -> crate::Result<Self> {
        let db = redb::Database::create(path)?;
        Ok(Self { db })
    }
}
