extern crate serde_json;

use crate::entries::*;
use crate::parse;
use glob::glob;
use regex::Regex;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn from_glob_filter<F, A>(
    g: &str,
    filter: F,
    mut action: A,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&PathBuf) -> bool,
    A: FnMut(PathBuf) -> Result<(), Box<dyn std::error::Error>>,
{
    for entry in glob(g)?.filter_map(Result::ok).filter(filter) {
        // dbg!(&entry);
        action(entry)?;
    }
    Ok(())
}

pub fn from_glob_regex<F>(
    g: &str,
    regex: Regex,
    action: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(PathBuf) -> Result<(), Box<dyn std::error::Error>>,
{
    from_glob_filter(g, |e| regex.is_match(e.to_str().unwrap()), action)
}

pub fn from_glob<F>(g: &str, action: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(PathBuf) -> Result<(), Box<dyn std::error::Error>>,
{
    from_glob_filter(g, |_| true, action)
}

pub fn translate(entry: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = PathBuf::new();
    out.push("json");
    out.push(entry.file_name().unwrap());
    out.set_extension("json");
    write(parse::read_from_file(entry.as_path())?, out.as_path())
}

pub fn write(
    result: (Vec<TeamEntry>, Vec<QuizzerEntry>),
    file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer(&File::create(file)?, &result)?;
    Ok(())
}

pub fn read(
    file: &Path,
) -> Result<(Vec<TeamEntry>, Vec<QuizzerEntry>), Box<dyn std::error::Error>> {
    // let json: (Vec<TeamEntry>, Vec<QuizzerEntry>) = serde_json::from_reader(&File::open(file)?)?;
    let json = serde_json::from_reader(&File::open(file)?)?;
    Ok(json)
}
