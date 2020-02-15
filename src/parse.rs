pub struct Id3Tag {
    // version: Id3Version,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<u32>,
    pub duration: Option<u64>,
    pub genre: Option<String>,
    pub disc: Option<u32>,
    // date_recorded: Option<Id3TimeStamp>,
    // date_released: Option<Id3TimeStamp>,
}

// pub enum Id3Version {
//     Id3v1,
//     Id3v22,
//     Id3v23,
//     Id3v24,
// }

// pub struct Id3TimeStamp {
//     year: Option<i32>,
//     month: Option<u8>,
//     day: Option<u8>,
//     hour: Option<u8>,
//     minute: Option<u8>,
//     second: Option<u8>,
// }

impl Id3Tag {
    pub fn new() -> Id3Tag {
        Id3Tag {
            // version,
            title: None,
            album: None,
            artist: None,
            year: None,
            track_number: None,
            duration: None,
            genre: None,
            disc: None,
            // date_recorded: None,
            // date_released: None,
        }
    }
}

impl From<id3::Tag> for Id3Tag {  
    fn from(source_tag: id3::Tag) -> Self {    
        Id3Tag {
            // version,
            title: source_tag.title().map(|s| s.to_string()),
            album: source_tag.album().map(|s| s.to_string()),
            artist: source_tag.artist().map(|s| s.to_string()),
            year: source_tag.year(),
            track_number: source_tag.track(),
            duration: source_tag.duration().map(|d| d as u64),
            genre: source_tag.genre().map(|s| s.to_string()),
            disc: source_tag.disc(),
            // date_recorded: source_tag.date_recorded(),
            // date_released: source_tag.date_released(),
        }
    }
}
