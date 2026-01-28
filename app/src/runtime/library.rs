use anyhow::Context;
use music_organizer_core::audio::identify::SongIdentifier;
use music_organizer_core::database::info::SongInfo;
use music_organizer_core::source::local::LocalSongSource;
use music_organizer_core::source::SongSource;

pub struct Library {
    tokio: tokio::runtime::Handle,
    identifier: Option<SongIdentifier>,
    source: Option<Box<dyn SongSource>>,
}

impl Library {
    pub fn new(tokio: &tokio::runtime::Handle) -> Self {
        Self {
            tokio: tokio.clone(),
            identifier: None,
            source: None,
        }
    }

    pub fn song_infos(&self) -> Option<impl Iterator<Item = &SongInfo>> {
        self.source.as_ref().map(|_archive| [].iter())
    }

    pub fn attach_local(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let source = LocalSongSource::attach(path)
            .context("Failed to open database, make sure it was created with Music Organizer")?;
        self.source = Some(Box::new(source));
        Ok(())
    }
}
