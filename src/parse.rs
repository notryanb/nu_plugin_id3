use id3::Tag as Id3Tag;

use nu_errors::ShellError;
use nu_protocol::{Primitive, TaggedDictBuilder, UntaggedValue, Value};

pub fn parse_id3_tag(value: Value) -> Result<Value, ShellError> {
    match &value.value {
        UntaggedValue::Primitive(Primitive::String(s)) => {
            let tag = Id3Tag::read_from_path(s);

            match tag {
                Ok(tag) => {
                    let mut dict = TaggedDictBuilder::new(&value.tag);

                    dict.insert_untagged(
                        "title",
                        tag.title().map_or_else(
                            || UntaggedValue::nothing(),
                            |title| UntaggedValue::string(title),
                        ),
                    );

                    dict.insert_untagged(
                        "album",
                        tag.album().map_or_else(
                            || UntaggedValue::nothing(),
                            |album| UntaggedValue::string(album),
                        ),
                    );

                    dict.insert_untagged(
                        "artist",
                        tag.artist().map_or_else(
                            || UntaggedValue::nothing(),
                            |artist| UntaggedValue::string(artist),
                        ),
                    );

                    dict.insert_untagged(
                        "year",
                        tag.year().map_or_else(
                            || UntaggedValue::nothing(),
                            |year| UntaggedValue::int(year),
                        ),
                    );

                    dict.insert_untagged(
                        "track number",
                        tag.track().map_or_else(
                            || UntaggedValue::nothing(),
                            |track| UntaggedValue::int(track),
                        ),
                    );

                    // dict.insert_untagged(
                    //     "duration",
                    //     tag.duration().map_or_else(|| UntaggedValue::nothing(), |duration| UntaggedValue::duration(duration as u64))
                    // );

                    dict.insert_untagged(
                        "genre",
                        tag.genre().map_or_else(
                            || UntaggedValue::nothing(),
                            |genre| UntaggedValue::string(genre),
                        ),
                    );

                    dict.insert_untagged(
                        "disc",
                        tag.disc().map_or_else(
                            || UntaggedValue::nothing(),
                            |disc| UntaggedValue::int(disc),
                        ),
                    );

                    let pictures = tag.pictures();
                    let mut pictures_dict = TaggedDictBuilder::new(&value.tag);

                    for pic in pictures.into_iter() {
                        pictures_dict
                            .insert_untagged("mime type", UntaggedValue::string(&pic.mime_type));

                        pictures_dict.insert_untagged(
                            "picture type",
                            UntaggedValue::string(picture_type_to_string(pic.picture_type)),
                        );

                        pictures_dict.insert_untagged(
                            "description",
                            UntaggedValue::string(&pic.description),
                        );

                        pictures_dict
                            .insert_untagged("data", UntaggedValue::binary(pic.data.clone()));
                    }

                    if pictures_dict.is_empty() {
                        dict.insert_untagged("pictures", UntaggedValue::nothing());
                    } else {
                        dict.insert_value("pictures", pictures_dict.into_value());
                    }

                    Ok(dict.into_value())
                }
                Err(_err) => {
                    let mut dict = TaggedDictBuilder::with_capacity(&value.tag, 8);

                    let columns = vec![
                        "title",
                        "album",
                        "artist",
                        "year",
                        "track number",
                        "duration",
                        "genre",
                        "disc",
                        "pictures",
                    ];

                    for col in columns {
                        dict.insert_untagged(col, UntaggedValue::nothing());
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
