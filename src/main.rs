mod parse;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{CallInfo, ReturnSuccess, ReturnValue, Signature, SyntaxShape, Value};
use nu_source::{Tag, Tagged, TaggedItem};
use std::path::Path;

use parse::parse_id3_tag;

struct Id3 {
    pub tag: Tag,
    pub filenames: Vec<Tagged<String>>,
}

impl Id3 {
    fn new() -> Id3 {
        Id3 {
            tag: Tag::unknown(),
            filenames: vec![],
        }
    }

    fn get_path_to_search<'a>(&self, call_info: &'a CallInfo) -> Value {
        if let Some(path) = call_info.args.nth(0) {
            path.to_owned()
        } else {
            Value::from(".")
        }
    }

    fn id3(&self, value: Value) -> Result<Value, ShellError> {
        parse_id3_tag(value)
    }

    fn parse_filenames(&mut self, call_info: &CallInfo) -> Result<(), ShellError> {
        let path = self.get_path_to_search(call_info);

        let mut results = vec![];

        let res = self.glob_to_values(&path)?;
        results.extend(res);

        if results.is_empty() {
            return Err(ShellError::labeled_error(
                "No filename(s) given",
                "no filename(s) given",
                self.tag.span,
            ));
        }

        for candidate in results {
            self.add_filename(candidate)?;
        }

        Ok(())
    }

    fn glob_to_values(&self, value: &Value) -> Result<Vec<Tagged<String>>, ShellError> {
        let mut result = vec![];

        for entry in walkdir::WalkDir::new(&value.as_string()?)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                let tagged_path = entry
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .tagged(value.tag.clone());
                result.push(tagged_path);
            }
        }

        Ok(result)
    }

    fn add_filename(&mut self, filename: Tagged<String>) -> Result<(), ShellError> {
        if Path::new(&filename.item).exists() || url::Url::parse(&filename.item).is_ok() {
            self.filenames.push(filename);
            Ok(())
        } else {
            Err(ShellError::labeled_error(
                format!("The file '{}' does not exist", filename.item),
                "doesn't exist",
                filename.tag,
            ))
        }
    }
}

impl Plugin for Id3 {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("id3")
            .desc("Display Id3 tag information for mp3 files")
            .optional(
                "path",
                SyntaxShape::GlobPattern,
                "The path to try and read all id3 tags from",
            )
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        self.tag = call_info.name_tag.clone();
        self.parse_filenames(&call_info)?;
        dbg!("ending begin filter");
        Ok(vec![])
    }

    fn filter(&mut self, _input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        let mut return_successes = vec![];
        dbg!("In filter");
        dbg!(&self.filenames.len());
        dbg!(&self.filenames);

        for filename in &self.filenames {
            let rs = ReturnSuccess::value(filename.item().clone());
            let inner_value = rs.unwrap().raw_value();

            dbg!("Inner Value");
            dbg!(&inner_value);

            if let Some(some_filename) = inner_value {
                let id3 = self.id3(some_filename)?;
                let value = ReturnSuccess::value(id3);
                dbg!("Value");
                dbg!(&value);
                return_successes.push(value);
            }
        }

        Ok(return_successes)
    }
}

fn main() {
    serve_plugin(&mut Id3::new());
}
