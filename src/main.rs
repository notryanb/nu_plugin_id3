use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;

struct Id3;

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn id3(&mut self, value: Value) -> Result<Value, ShellError> {
        match &value.value {
            UntaggedValue::Primitive(Primitive::String(s)) => {
                dbg!(s);
                let tag = Id3Tag::read_from_path(s).expect("Couldn't read file");

                Ok(Value {
                    value: tag.artist().unwrap_or("failed artist").into(),
                    tag: value.tag,
                })
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
        Ok(Signature::build("id3").filter())
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
