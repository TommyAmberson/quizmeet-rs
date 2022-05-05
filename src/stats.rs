use crate::{entries::*, io::*};
use regex::Regex;
use std::collections::HashMap;

pub fn open_json(
    g: Option<String>,
    re: Option<Regex>,
) -> Result<(HashMap<String, TeamEntry>, HashMap<String, QuizzerEntry>), Box<dyn std::error::Error>>
{
    let g = g.unwrap_or(String::from("json/*.json"));
    let re = re.unwrap_or(Regex::new(r"^json/D(?P<d>\dC?)Q(?P<q>(\d|\w)+).json$")?);
    let mut team_entries: Vec<TeamEntry> = Vec::new();
    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    from_glob_regex(&g, re, |entry| {
        let result = read(entry.as_path())?;
        // dbg!(&result);
        team_entries.extend(result.0);
        quizzer_entries.extend(result.1);
        Ok(())
    })?;
    let team_sums = group_and_sum(team_entries)?;
    let quizzer_sums = group_and_sum(quizzer_entries)?;
    Ok((team_sums, quizzer_sums))
}
