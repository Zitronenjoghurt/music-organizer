#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, bitcode::Encode, bitcode::Decode)]
#[repr(u8)]
pub enum SongFileFormat {
    Mp3 = 0,
    M4a = 1,
    Ogg = 2,
    Wav = 3,
    Opus = 4,
    Aiff = 5,
    Flac = 6,
}

impl SongFileFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::M4a => "m4a",
            Self::Ogg => "ogg",
            Self::Wav => "wav",
            Self::Opus => "opus",
            Self::Aiff => "aiff",
            Self::Flac => "flac",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "mp3" => Some(Self::Mp3),
            "m4a" => Some(Self::M4a),
            "ogg" => Some(Self::Ogg),
            "wav" => Some(Self::Wav),
            "opus" => Some(Self::Opus),
            "aiff" => Some(Self::Aiff),
            "flac" => Some(Self::Flac),
            _ => None,
        }
    }
}
