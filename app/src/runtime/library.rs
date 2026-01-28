use anyhow::Context;
use music_organizer_core::archive::Archive;
use music_organizer_core::types::song_info::SongInfo;

#[derive(Default)]
pub struct Library {
    archive: Option<Archive>,
}

impl Library {
    pub fn song_infos(&self) -> Option<impl Iterator<Item = &SongInfo>> {
        self.archive.as_ref().map(|_archive| [].iter())
    }

    pub fn attach(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.archive = Some(
            Archive::attach(path)
                .context("Failed to open archive, make sure it was created with Music Organizer")?,
        );
        Ok(())
    }
}
