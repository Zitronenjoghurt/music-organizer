use crate::database::entry::SongEntry;
use crate::database::info::SongInfo;
use redb::{ReadableDatabase, ReadableTable, TableDefinition};
use std::collections::HashMap;
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

        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(SONGS)?;
        }
        write_txn.commit()?;

        Ok(Self { db })
    }

    pub fn insert_entry(&self, entry: SongEntry) -> crate::Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(SONGS)?;
            table.insert(entry.file.hash, entry.to_bytes())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn song_infos(&self) -> crate::Result<HashMap<u64, SongInfo>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(SONGS)?;

        let mut map = HashMap::new();

        for item in table.iter()? {
            let (key, value) = item?;
            if let Ok(entry) = SongEntry::from_bytes(&value.value()) {
                map.insert(key.value(), entry.info);
            }
        }

        Ok(map)
    }
}
