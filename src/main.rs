mod parse;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, TaggedDictBuilder, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;
use parse::Id3Tag as MyTag;

struct Id3;

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn id3(&mut self, value: Value) -> Result<Value, ShellError> {
        let nu_tag = value.tag.clone();

        match &value.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                let tag = Id3Tag::read_from_path(s);


                match tag {
                    Ok(tag) =>  Ok(MyTag::from(tag).into_value(&nu_tag)),
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
