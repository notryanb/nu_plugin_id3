use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, TaggedDictBuilder, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;

struct Id3;

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn idthree(&mut self, value: Value) -> Result<Value, ShellError> {
        match &value.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                let tag = Id3Tag::read_from_path(s).expect("Couldn't read file");
                let mut dict = TaggedDictBuilder::with_capacity(&value.tag, 3);

                dict.insert_untagged(
                    "artist",
                    UntaggedValue::string(tag.artist().unwrap_or("failed artist").to_string())
                );

                dict.insert_untagged(
                    "album",
                    UntaggedValue::string(tag.album().unwrap_or("failed album").to_string())
                );

                dict.insert_untagged(
                    "title",
                    UntaggedValue::string(tag.title().unwrap_or("failed title").to_string())
                );

                Ok(dict.into_value())
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
