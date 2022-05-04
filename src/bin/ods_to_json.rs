extern crate serde_json;

use clap::Parser;
use glob::glob;
use quizmeet_rs::{entries::QuizEntry, parse};
use std::fs::File;
use std::path::{Path,PathBuf};

#[derive(Parser)]
struct Cli {
    g: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let g = args.g.unwrap_or(String::from("ods/*.ods"));
    match from_glob(&g) {
        Ok(_result) => {
            // dbg!(result);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

pub fn from_glob(g: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in glob(g)?.filter_map(Result::ok) {
        let mut out = PathBuf::new();
        out.push("json");
        out.push(entry.file_name().unwrap());
        out.set_extension("json");
        translate(entry.as_path(), out.as_path())?;
    }
    Ok(())
}

fn translate(read: &Path, write: &Path) -> Result<QuizEntry, Box<dyn std::error::Error>> {
    // let mut path = PathBuf::from(p);
    dbg!(&read);
    let result = parse::read_from_file(read)?;
    dbg!(&write);
    ::serde_json::to_writer(&File::create(write)?, &result)?;
    Ok(result)
}
