use crate::audio::identify::SongIdentifier;
use crate::database::entry::SongEntry;
use crate::database::info::SongInfo;
use crate::database::Database;
use crate::source::SongSource;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub struct LocalSongSource {
    db: Database,
    identifier: Option<SongIdentifier>,
}

impl LocalSongSource {
    pub fn attach(path: impl AsRef<Path>) -> crate::Result<Self> {
        let db = Database::attach(path)?;
        Ok(Self {
            db,
            identifier: None,
        })
    }
}

#[async_trait::async_trait]
impl SongSource for LocalSongSource {
    async fn set_acoustid_key(&mut self, key: &str) -> crate::Result<()> {
        self.identifier = Some(SongIdentifier::new(key)?);
        Ok(())
    }

    async fn import_song(&self, path: &Path) -> crate::Result<()> {
        let entry = if let Some(identifier) = &self.identifier {
            identifier.identify_song(path).await?
        } else {
            SongEntry::from_file(SongIdentifier::song_file(path)?)
        };

        self.db.insert_entry(entry)?;

        Ok(())
    }

    async fn get_song_infos(&self) -> crate::Result<HashMap<u64, SongInfo>> {
        Ok(self.db.song_infos()?)
    }
}
