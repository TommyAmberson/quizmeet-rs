use quizmeet_rs::parse;
use std::path::Path;

fn main() {
    let result = parse::read_from_file(Path::new("ods/D1Q1.ods"));
    match result {
        Ok(result) => {
            dbg!(result);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
