use redb::TableDefinition;
use std::path::Path;

const SONG_DATA: TableDefinition<u64, Vec<u8>> = TableDefinition::new("my_data");

pub struct Archive {
    db: redb::Database,
}

impl Archive {
    pub fn attach(path: impl AsRef<Path>) -> crate::Result<Self> {
        let db = redb::Database::create(path)?;
        Ok(Self { db })
    }

    pub fn import_from_file(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        Ok(())
    }
}
