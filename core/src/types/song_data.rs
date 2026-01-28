use crate::types::song_file_info::SongFileInfo;

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct SongData {
    pub file_info: SongFileInfo,
    pub data: Vec<u8>,
}
