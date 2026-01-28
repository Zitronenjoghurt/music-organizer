use crate::song_source::SongSource;
use crate::types::song_info::SongInfo;
use redb::TableDefinition;
use std::collections::HashMap;
use std::path::Path;

const SONG_DATA: TableDefinition<u64, Vec<u8>> = TableDefinition::new("song_data");
const SONG_INFO: TableDefinition<u64, Vec<u8>> = TableDefinition::new("song_info");

pub struct Archive {
    db: redb::Database,
}

impl Archive {
    pub fn attach(path: impl AsRef<Path>) -> crate::Result<Self> {
        let db = redb::Database::create(path)?;
        Ok(Self { db })
    }
}

#[async_trait::async_trait]
impl SongSource for Archive {
    async fn get_song_infos(&self) -> Result<HashMap<u64, SongInfo>, Box<dyn std::error::Error>> {
        Ok(HashMap::new())
    }
}
