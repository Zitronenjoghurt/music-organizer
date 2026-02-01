use crate::app::actions::AppActions;
use crate::runtime::task::AsyncTask;
use music_organizer_core::database::info::SongInfo;
use music_organizer_core::error::Error;
use music_organizer_core::source::local::LocalSongSource;
use music_organizer_core::source::SongSource;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Library {
    tokio: tokio::runtime::Handle,
    source: Option<Arc<dyn SongSource>>,
    pub song_infos: AsyncTask<Arc<dyn SongSource>, HashMap<u64, SongInfo>, Error>,
}

impl Library {
    pub fn new(tokio: &tokio::runtime::Handle) -> Self {
        Self {
            tokio: tokio.clone(),
            source: None,
            song_infos: AsyncTask::new(tokio, |source: Arc<dyn SongSource>| async move {
                source.get_song_infos().await
            }),
        }
    }

    pub fn update(&mut self, actions: &AppActions) {
        if let Some(error) = self.song_infos.take_error() {
            actions.toast_error(error.to_string());
        }
    }

    pub fn attach_local(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let source = LocalSongSource::attach(path)?;
        let source = Arc::new(source);
        self.source = Some(source.clone());

        self.song_infos.trigger(source);

        Ok(())
    }
}
