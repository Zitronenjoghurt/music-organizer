#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SongFile {
    pub name: String,
    pub path: String,
    pub hash: u64,
}
