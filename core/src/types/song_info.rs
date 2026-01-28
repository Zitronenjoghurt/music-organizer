#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SongInfo {
    pub name: String,
    pub artist: String,
}
