use std::path::Path;

pub fn hash_file(path: impl AsRef<Path>) -> crate::Result<u64> {
    let contents = std::fs::read(path)?;
    let hash = blake3::hash(&contents);
    let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
    Ok(u64::from_le_bytes(bytes))
}
