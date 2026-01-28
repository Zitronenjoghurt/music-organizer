use crate::database::entry::SongEntry;
use crate::database::file::SongFile;
use crate::database::info::SongInfo;
use crate::error::Error;
use crate::utils::hash_file;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::Fetch;
use std::path::Path;

pub mod acoustid;
pub mod fingerprint;

pub struct SongIdentifier {
    acoustid: acoustid::AcoustID,
    // ToDo: add LastFM as a secondary source of information
}

impl SongIdentifier {
    pub fn new(acoust_id_key: impl AsRef<str>) -> crate::Result<Self> {
        Ok(Self {
            acoustid: acoustid::AcoustID::new(acoust_id_key)?,
        })
    }

    pub async fn identify_song(&self, path: impl AsRef<Path>) -> crate::Result<SongEntry> {
        let file = Self::song_file(&path)?;

        let (fp, duration) = fingerprint::song_file_fingerprint(path)?;
        let Some(musicbrainz_id) = self.acoustid.lookup(fp, duration).await? else {
            return Ok(SongEntry { file, info: None });
        };

        let info = Self::song_info(&musicbrainz_id).await?;
        Ok(SongEntry { file, info })
    }

    fn song_file(path: impl AsRef<Path>) -> crate::Result<SongFile> {
        let path = path.as_ref().canonicalize()?;
        let hash = hash_file(&path)?;
        let name = path
            .file_name()
            .ok_or(Error::InvalidFilePath)?
            .to_string_lossy()
            .to_string();
        Ok(SongFile {
            name,
            path: path.to_string_lossy().to_string(),
            hash,
        })
    }

    async fn song_info(musicbrainz_id: &str) -> crate::Result<Option<SongInfo>> {
        let recording = match Recording::fetch()
            .id(musicbrainz_id)
            .with_artists()
            .with_releases()
            .with_genres()
            .with_tags()
            .with_aliases()
            .with_ratings()
            .execute()
            .await
        {
            Ok(recording) => recording,
            Err(err) => {
                return if matches!(err, musicbrainz_rs::error::Error::NotFound(_)) {
                    Ok(None)
                } else {
                    Err(err.into())
                };
            }
        };

        let aliases = if let Some(aliases) = recording.aliases {
            aliases.iter().map(|alias| alias.name.clone()).collect()
        } else {
            vec![]
        };

        let artists = if let Some(credits) = recording.artist_credit {
            credits
                .iter()
                .map(|credit| credit.artist.name.clone())
                .collect()
        } else {
            vec![]
        };

        let genres = if let Some(genres) = recording.genres {
            genres
                .iter()
                .map(|genre| (genre.name.clone(), genre.count.unwrap_or_default()))
                .collect()
        } else {
            vec![]
        };

        let tags = if let Some(tags) = recording.tags {
            tags.iter()
                .map(|tag| {
                    (
                        tag.name.clone(),
                        tag.count.unwrap_or_default(),
                        tag.score.unwrap_or_default(),
                    )
                })
                .collect()
        } else {
            vec![]
        };

        Ok(Some(SongInfo {
            musicbrainz_id: recording.id.clone(),
            title: recording.title.clone(),
            aliases,
            artists,
            first_release: recording.first_release_date.map(|date| date.0),
            genres,
            tags,
        }))
    }
}
