use crate::database::info::SongInfo;
use crate::database::Database;
use crate::source::SongSource;
use std::collections::HashMap;
use std::error::Error;

pub struct LocalSongSource {
    db: Database,
}

impl LocalSongSource {
    pub fn attach(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let db = Database::attach(path)?;
        Ok(Self { db })
    }
}

#[async_trait::async_trait]
impl SongSource for LocalSongSource {
    async fn get_song_infos(&self) -> Result<HashMap<u64, SongInfo>, Box<dyn Error>> {
        todo!()
    }
}
