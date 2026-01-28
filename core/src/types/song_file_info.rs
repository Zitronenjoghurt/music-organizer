use crate::types::song_file_format::SongFileFormat;
use crate::types::song_info::SongInfo;

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct SongFileInfo {
    pub song_info: SongInfo,
    pub format: SongFileFormat,
}
