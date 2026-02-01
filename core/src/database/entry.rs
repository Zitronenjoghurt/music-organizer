use crate::database::file::SongFile;
use crate::database::info::SongInfo;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SongEntry {
    pub file: SongFile,
    pub info: SongInfo,
}

impl SongEntry {
    pub fn from_file(file: SongFile) -> Self {
        Self {
            file,
            info: Default::default(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.into()
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        Self::try_from(bytes.to_vec())
    }
}

impl From<&SongEntry> for Vec<u8> {
    fn from(value: &SongEntry) -> Self {
        bitcode::encode(value)
    }
}

impl TryFrom<Vec<u8>> for SongEntry {
    type Error = crate::error::Error;

    fn try_from(value: Vec<u8>) -> crate::Result<Self> {
        Ok(bitcode::decode(&value)?)
    }
}
