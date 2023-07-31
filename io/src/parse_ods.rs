use std::path::Path;

use anyhow::{bail, Result};
use quizmeet_rs_idl::quiz::{Quiz, QuizzerEntry, TeamEntry};
use spreadsheet_ods::Sheet;

pub fn read_from_file(path: &Path) -> Result<Quiz> {
    log::debug!("reading path: {path:?}");

    let wb = spreadsheet_ods::read_ods(path)?;
    if wb.num_sheets() < 2 {
        bail!("must have at least two sheets");
    }
    let sheet = wb.sheet(1);

    let mut team_entries: Vec<TeamEntry> = Vec::new();
    for row in 1..4 {
        match team(sheet, row) {
            Ok(team) => {
                log::trace!("read team: '{team:?}' from '{path:?}'");
                team_entries.push(team)
            }
            Err(e) => {
                log::warn!("skipping row: '{row}' of '{path:?}' because: '{e}'");
                continue;
            }
        };
    }

    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    for row in 6..21 {
        match quizzer(sheet, row) {
            Ok(quizzer) => {
                log::trace!("read quizzer: '{quizzer:?}' from '{path:?}'");
                quizzer_entries.push(quizzer)
            }
            Err(e) => {
                log::warn!("skipping row: '{row}' of '{path:?}' because: '{e}'");
                continue;
            }
        };
    }

    let quiz = Quiz {
        team_entries,
        quizzer_entries,
    };
    log::debug!("read quiz: '{quiz:?}' from {path:?}");
    Ok(quiz)
}

fn parse_error(sheet: &Sheet, row: u32, col: u32, val: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "Failed to parse '{}' for 'row{}:col{}' of sheet: '{:?}'",
        val,
        row,
        col,
        sheet
    )
}

fn parse_str(sheet: &Sheet, row: u32, col: u32, val: &str) -> Result<String> {
    sheet
        .value(row, col)
        .as_str_opt()
        .ok_or_else(|| parse_error(sheet, row, col, val))
        .map(|s| s.to_owned())
}
fn parse_i32(sheet: &Sheet, row: u32, col: u32, val: &str) -> Result<i32> {
    sheet
        .value(row, col)
        .as_i32_opt()
        .ok_or_else(|| parse_error(sheet, row, col, val))
}
fn parse_f64(sheet: &Sheet, row: u32, col: u32, val: &str) -> Result<f64> {
    sheet
        .value(row, col)
        .as_f64_opt()
        .ok_or_else(|| parse_error(sheet, row, col, val))
}

fn team(sheet: &Sheet, row: u32) -> Result<TeamEntry> {
    Ok(TeamEntry {
        name: parse_str(sheet, row, 0, "name")?,
        quiz: parse_str(sheet, row, 1, "quiz")?,
        place: parse_f64(sheet, row, 2, "place")?,
        score: parse_i32(sheet, row, 3, "score")?,
        points: parse_i32(sheet, row, 4, "points")?,
        errors: parse_i32(sheet, row, 5, "errors")?,
    })
}

fn quizzer(sheet: &Sheet, row: u32) -> Result<QuizzerEntry> {
    Ok(QuizzerEntry {
        name: parse_str(sheet, row, 0, "name")?,
        team: parse_str(sheet, row, 0, "team")?,
        quiz: parse_str(sheet, row, 1, "quiz")?,
        points: parse_i32(sheet, row, 4, "points")?,
        errors: parse_i32(sheet, row, 5, "errors")?,
        jumps: parse_i32(sheet, row, 6, "jumps")?,
        refer: parse_i32(sheet, row, 7, "refer")?,
        ftv: parse_i32(sheet, row, 8, "ftv")?,
        int: parse_i32(sheet, row, 9, "int")?,
        ma: parse_i32(sheet, row, 10, "ma")?,
        q: parse_i32(sheet, row, 11, "q")?,
        sit: parse_i32(sheet, row, 12, "sit")?,
    })
}
