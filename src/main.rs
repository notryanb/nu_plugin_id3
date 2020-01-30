use nu::{
    serve_plugin, 
    CallInfo, 
    Plugin, 
    Primitive, 
    ReturnSuccess, 
    ReturnValue, 
    ShellError, 
    Signature,
    Tagged, 
    Value,
};

struct Id3;

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn id3(&mut self, value: Tagged<Value>) -> Result<Tagged<Value>, ShellError> {
        use id3::{Tag, Version};

        match value.item {
            Value::Primitive(Primitive::String(s)) => {
                let mut tag = Tag::read_from_path(s).unwrap();

                Ok(Tagged {
                    item: Value::int(s.len() as i64),
                    tag: value.tag,
                })
            },
            _ => Err(ShellError::labeled_error(
                    "Unrecognized type in stream",
                    "'len' given non-string by this",
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

    fn filter(&mut self, input: Tagged<Value>) -> Result<Vec<ReturnValue>, ShellError> {
        // Ok(vec![ReturnSuccess::value(self.id3(input)?)])
        Ok(vec![])
    }
}

fn main() {
    serve_plugin(&mut Id3::new());
}

