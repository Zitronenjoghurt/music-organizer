pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Audio processor error: {0}")]
    AudioProcessor(#[from] rusty_chromaprint::ResetError),
    #[error("Database error: {0}")]
    Database(#[from] redb::DatabaseError),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("No track found")]
    NoTrackFound,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Symphonia error: {0}")]
    Symphonia(#[from] symphonia::core::errors::Error),
}
