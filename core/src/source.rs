use crate::database::info::SongInfo;
use std::collections::HashMap;

pub mod local;

#[async_trait::async_trait]
pub trait SongSource {
    async fn get_song_infos(&self) -> Result<HashMap<u64, SongInfo>, Box<dyn std::error::Error>>;
}
