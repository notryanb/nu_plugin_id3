mod parse;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, TaggedDictBuilder, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;
use parse::Id3Tag as MyTag;


struct Id3;

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

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn idthree(&mut self, value: Value) -> Result<Value, ShellError> {
        match &value.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                let tag = Id3Tag::read_from_path(s);

                match tag {
                    Ok(tag) => {
                        let my_tag = MyTag::from(tag);

                        let mut dict = TaggedDictBuilder::with_capacity(&value.tag, 8);
                        // let pictures = tag.pictures();

                        // let mut pictures_dict = TaggedDictBuilder::new(&value.tag);

                        // for pic in pictures {
                        //     pictures_dict.insert_untagged(
                        //         "mime type",
                        //         UntaggedValue::string(&pic.mime_type)
                        //     );

                        //     pictures_dict.insert_untagged(
                        //         "picture type",
                        //         UntaggedValue::string(picture_type_to_string(pic.picture_type))
                        //     );

                        //     pictures_dict.insert_untagged(
                        //         "description",
                        //         UntaggedValue::string(&pic.description)
                        //     );

                        //     pictures_dict.insert_untagged(
                        //         "data",
                        //         UntaggedValue::binary(pic.data.clone())
                        //     );
                        // }

                        // dict.insert_value(
                        //     "pictures",
                        //     pictures_dict.into_value()
                        // );
        
                        dict.insert_untagged(
                            "title",
                            UntaggedValue::string(my_tag.title.unwrap_or(String::new()))
                        );
                        
                        dict.insert_untagged(
                            "album",
                            UntaggedValue::string(my_tag.album.unwrap_or(String::new()))
                        );
                        
                        dict.insert_untagged(
                            "artist",
                            UntaggedValue::string(my_tag.artist.unwrap_or(String::new()))
                        );
        
                        dict.insert_untagged(
                            "year",
                            UntaggedValue::int(my_tag.year.unwrap_or(0))
                        );
        
                        dict.insert_untagged(
                            "track number",
                            UntaggedValue::int(my_tag.track_number.unwrap_or(0))
                        );
        
                        dict.insert_untagged(
                            "duration",
                            UntaggedValue::duration(my_tag.duration.unwrap_or(0))
                        );

                        dict.insert_untagged(
                            "genre",
                            UntaggedValue::string(my_tag.genre.unwrap_or(String::new()))
                        );
        
                        dict.insert_untagged(
                            "disc",
                            UntaggedValue::int(my_tag.disc.unwrap_or(0))
                        );
        
                        // dict.insert_untagged(
                        //     "date released",
                        //     UntaggedValue::string(tag.date_released().unwrap_or(id3::Timestamp {
                        //         year: 0,
                        //         month: None,
                        //         day: None,
                        //         hour: None,
                        //         minute: None,
                        //         second: None,
                        //     }).to_string())
                        // );
        
                        // dict.insert_untagged(
                        //     "date recorded",
                        //     UntaggedValue::string(tag.date_recorded().unwrap_or(id3::Timestamp {
                        //         year: 0,
                        //         month: None,
                        //         day: None,
                        //         hour: None,
                        //         minute: None,
                        //         second: None,
                        //     }).to_string())
                        // );
        
                        Ok(dict.into_value())

                    }
                    Err(_err) => {
                        let mut dict = TaggedDictBuilder::with_capacity(&value.tag, 8);

                        let columns = vec![
                            "pictures",
                            "title",
                            "album",
                            "artist",
                            "year",
                            "track number",
                            "duration",
                            "genre",
                            "disc",
                        ];

                        for col in columns {
                            dict.insert_untagged(
                                col,
                                UntaggedValue::nothing()
                            );
                        }

                        Ok(dict.into_value())
                    }
                }
                
            }
            _ => Err(ShellError::labeled_error(
                "Unrecognized type in stream",
                "'id3' given non-string by this",
                value.tag.span,
            )),
        }
    }
}

impl Plugin for Id3 {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("idthree").desc("Display Id3 tag information for mp3 files").filter())
    }

    fn begin_filter(&mut self, _: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.idthree(input)?)])
    }
}

fn main() {
    serve_plugin(&mut Id3::new());
}
