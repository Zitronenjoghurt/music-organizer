use crate::database::info::SongInfo;
use std::collections::HashMap;
use std::path::Path;

pub mod local;

#[async_trait::async_trait]
pub trait SongSource: Send + Sync {
    async fn set_acoustid_key(&mut self, key: &str) -> crate::Result<()>;
    async fn import_song(&self, path: &Path) -> crate::Result<()>;
    async fn get_song_infos(&self) -> crate::Result<HashMap<u64, SongInfo>>;
}
