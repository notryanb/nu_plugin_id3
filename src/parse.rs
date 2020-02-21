// use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize)]
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
    // pub pictures: Vec<Picture>,
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


// impl Id3Tag {
    // pub fn new() -> Id3Tag {
    //     Id3Tag {
    //         // version,
    //         title: None,
    //         album: None,
    //         artist: None,
    //         year: None,
    //         track_number: None,
    //         duration: None,
    //         genre: None,
    //         disc: None,
    //         // pictures: Vec::new(),
    //         // date_recorded: None,
    //         // date_released: None,
    //     }
    // }
// }

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
            // pictures: source_tag.pictures().map::<id3::frame::Picture, Picture>(|p| Picture::from(p.into())).collect::<Vec<Picture>>(),
            // date_recorded: source_tag.date_recorded(),
            // date_released: source_tag.date_released(),
        }
    }
}


pub struct Picture {
    pub mime_type: String,
    pub picture_type: String,
    pub description: String,
    pub data: Vec<u8>,
}

impl From<id3::frame::Picture> for Picture {  
    fn from(source_picture: id3::frame::Picture) -> Self {    
        Picture {
            mime_type: source_picture.mime_type,
            picture_type: picture_type_to_string(source_picture.picture_type),
            description: source_picture.description,
            data: source_picture.data,
        }
    }
}

fn picture_type_to_string(picture_type: id3::frame::PictureType) -> String {
    let pic_type_str = match picture_type {
        id3::frame::PictureType::Other => "other",
        id3::frame::PictureType::Icon => "icon",
        id3::frame::PictureType::OtherIcon => "other icon",
        id3::frame::PictureType::CoverFront => "front cover",
        id3::frame::PictureType::CoverBack => "back cover",
        id3::frame::PictureType::Leaflet => "leaflet",
        id3::frame::PictureType::Media => "media",
        id3::frame::PictureType::LeadArtist => "lead artist",
        id3::frame::PictureType::Artist => "artist",
        id3::frame::PictureType::Conductor => "conductor",
        id3::frame::PictureType::Band => "band",
        id3::frame::PictureType::Composer => "composer",
        id3::frame::PictureType::Lyricist => "lyricist",
        id3::frame::PictureType::RecordingLocation => "recording location",
        id3::frame::PictureType::DuringRecording => "during recording",
        id3::frame::PictureType::DuringPerformance => "during performance",
        id3::frame::PictureType::ScreenCapture => "screen capture",
        id3::frame::PictureType::BrightFish => "bright fish",
        id3::frame::PictureType::Illustration => "illustration",
        id3::frame::PictureType::BandLogo => "band logo",
        id3::frame::PictureType::PublisherLogo => "publisher logo",
        _ => "undefined",
    };

    pic_type_str.to_string()
}
