use crate::database::file::SongFile;
use crate::database::info::SongInfo;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SongEntry {
    pub file: SongFile,
    pub info: Option<SongInfo>,
}
