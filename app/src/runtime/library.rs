use anyhow::Context;
use music_organizer_core::archive::Archive;
use music_organizer_core::audio::identify::acoustid::AcoustID;
use music_organizer_core::song_source::SongSource;
use music_organizer_core::types::song_info::SongInfo;

#[derive(Default)]
pub struct Library {
    acoust_id: Option<AcoustID>,
    source: Option<Box<dyn SongSource>>,
}

impl Library {
    pub fn song_infos(&self) -> Option<impl Iterator<Item = &SongInfo>> {
        self.source.as_ref().map(|_archive| [].iter())
    }

    pub fn attach_local(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let archive = Archive::attach(path)
            .context("Failed to open archive, make sure it was created with Music Organizer")?;
        self.source = Some(Box::new(archive));
        Ok(())
    }
}
