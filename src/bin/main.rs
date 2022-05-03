extern crate serde_json;

use std::fs::File;

use quizmeet_rs::{entries::QuizEntry, parse};
use std::path::Path;

fn main() {
    match write() {
        Ok(result) => {
            dbg!(result);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

fn write() -> Result<QuizEntry, Box<dyn std::error::Error>> {
    let result = parse::read_from_file(Path::new("ods/D1Q1.ods"))?;
    ::serde_json::to_writer(&File::create("data.json")?, &result)?;
    Ok(result)
}
