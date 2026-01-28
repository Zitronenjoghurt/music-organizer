#[derive(bitcode::Encode, bitcode::Decode)]
pub struct SongData {
    pub data: Vec<u8>,
}
