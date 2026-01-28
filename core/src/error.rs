pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Audio processor error: {0}")]
    AudioProcessor(#[from] rusty_chromaprint::ResetError),
    #[error("Database error: {0}")]
    Database(#[from] redb::DatabaseError),
    #[error("Invalid file path")]
    InvalidFilePath,
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("MusicBrainz error: {0}")]
    MusicBrainz(#[from] musicbrainz_rs::Error),
    #[error("No track found")]
    NoTrackFound,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Symphonia error: {0}")]
    Symphonia(#[from] symphonia::core::errors::Error),
}
