mod parse;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, TaggedDictBuilder, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;
use parse::Id3Tag as MyTag;


#[derive(Default)]
struct Id3 {
    id3s: Vec<Value>
}

impl Id3 {
    fn new() -> Id3 {
        Id3 {
            id3s: Vec::new()
        }
    }

    fn id3(&mut self, value: Value) -> Result<Value, ShellError> {
        let nu_tag = value.tag.clone();

        match &value.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                let tag = Id3Tag::read_from_path(s);


                match tag {
                    Ok(tag) => {
                        let my_tag = MyTag::from(tag);
                        self.id3s.push(my_tag.into());

                        Ok(Value { value: UntaggedValue::Table(self.id3s), tag: nu_tag })

                        // match my_tag.to_json() {
                        //     Ok(json) => {

                        //         Ok(json.into())
                        //     },
                        //     _ => Err(ShellError::labeled_error(
                        //         "Unrecognized type in stream",
                        //         "'id3' given non-string by this",
                        //         value.tag.span,
                        //     ))
                        // }

                        // let mut dict = TaggedDictBuilder::with_capacity(&value.tag, 8);
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
        
                        // dict.insert_untagged(
                        //     "title",
                        //     UntaggedValue::string(my_tag.title.unwrap_or(String::new()))
                        // );
                        
                        // dict.insert_untagged(
                        //     "album",
                        //     UntaggedValue::string(my_tag.album.unwrap_or(String::new()))
                        // );
                        
                        // dict.insert_untagged(
                        //     "artist",
                        //     UntaggedValue::string(my_tag.artist.unwrap_or(String::new()))
                        // );
        
                        // dict.insert_untagged(
                        //     "year",
                        //     UntaggedValue::int(my_tag.year.unwrap_or(0))
                        // );
        
                        // dict.insert_untagged(
                        //     "track number",
                        //     UntaggedValue::int(my_tag.track_number.unwrap_or(0))
                        // );
        
                        // dict.insert_untagged(
                        //     "duration",
                        //     UntaggedValue::duration(my_tag.duration.unwrap_or(0))
                        // );

                        // dict.insert_untagged(
                        //     "genre",
                        //     UntaggedValue::string(my_tag.genre.unwrap_or(String::new()))
                        // );
        
                        // dict.insert_untagged(
                        //     "disc",
                        //     UntaggedValue::int(my_tag.disc.unwrap_or(0))
                        // );
        
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
        
                        // Ok(dict.into_value())

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
        Ok(Signature::build("id3").desc("Display Id3 tag information for mp3 files").filter())
    }

    fn begin_filter(&mut self, _: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.id3(input)?)])
    }
}

fn main() {
    serve_plugin(&mut Id3::new());
}
