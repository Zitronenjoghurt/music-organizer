#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SongInfo {
    pub musicbrainz_id: String,
    pub title: String,
    pub aliases: Vec<String>,
    pub artists: Vec<String>,
    pub first_release: Option<String>,
    pub genres: Vec<(String, u32)>,
    pub tags: Vec<(String, i32, i32)>,
}
