use crate::types::song_info::SongInfo;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait SongSource {
    async fn get_song_infos(&self) -> Result<HashMap<u64, SongInfo>, Box<dyn std::error::Error>>;
}
