use crate::entries::*;
use spreadsheet_ods::{error::OdsError, Sheet};
use std::path::Path;

pub fn read_from_file(
    path: &Path,
) -> Result<(Vec<TeamEntry>, Vec<QuizzerEntry>), Box<dyn std::error::Error>> {
    // dbg!(path.display());

    let wb = spreadsheet_ods::read_ods(path)?;
    if wb.num_sheets() < 2 {
        return Err(Box::new(OdsError::Ods(String::from(
            "must have at least two sheets",
        ))));
    }
    let sheet = wb.sheet(1);
    let mut team_entries: Vec<TeamEntry> = Vec::new();
    for row in 1..4 {
        let team = match team(&sheet, row) {
            Ok(team) => team,
            Err(_) => continue,
        };
        team_entries.push(team);
        // dbg!(&row);
        // dbg!(&team_entries);
    }
    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    for row in 6..21 {
        let quizzer = match quizzer(&sheet, row) {
            Ok(quizzer) => quizzer,
            Err(_) => continue,
        };
        quizzer_entries.push(quizzer);
        // dbg!(&row);
        // dbg!(&quizzer_entries);
    }
    Ok((team_entries, quizzer_entries))
}

fn team(sheet: &Sheet, row: u32) -> Result<TeamEntry, Box<dyn std::error::Error>> {
    Ok(TeamEntry {
        name: String::from(
            sheet
                .value(row, 0)
                .as_str_opt()
                .ok_or("failed to parse name")?,
        ),
        quiz: String::from(
            sheet
                .value(row, 1)
                .as_str_opt()
                .ok_or("failed to parse quiz")?,
        ),
        place: sheet
            .value(row, 2)
            .as_f64_opt()
            .ok_or("failed to parse place")?,
        score: sheet
            .value(row, 3)
            .as_i32_opt()
            .ok_or("failed to parse score")?,
        points: sheet
            .value(row, 4)
            .as_i32_opt()
            .ok_or("failed to parse points")?,
        errors: sheet
            .value(row, 5)
            .as_i32_opt()
            .ok_or("failed to parse errors")?,
    })
}

fn quizzer(sheet: &Sheet, row: u32) -> Result<QuizzerEntry, Box<dyn std::error::Error>> {
    Ok(QuizzerEntry {
        name: String::from(
            sheet
                .value(row, 0)
                .as_str_opt()
                .ok_or("failed to parse name")?,
        ),
        team: String::from(
            sheet
                .value(row, 1)
                .as_str_opt()
                .ok_or("failed to parse team")?,
        ),
        quiz: String::from(
            sheet
                .value(row, 2)
                .as_str_opt()
                .ok_or("failed to parse quiz")?,
        ),
        points: sheet
            .value(row, 3)
            .as_i32_opt()
            .ok_or("failed to parse points")?,
        errors: sheet
            .value(row, 4)
            .as_i32_opt()
            .ok_or("failed to parse errors")?,
        jumps: sheet
            .value(row, 5)
            .as_i32_opt()
            .ok_or("failed to parse jumps")?,
        refer: sheet
            .value(row, 6)
            .as_i32_opt()
            .ok_or("failed to parse refer")?,
        ftv: sheet
            .value(row, 7)
            .as_i32_opt()
            .ok_or("failed to parse ftv")?,
        int: sheet
            .value(row, 8)
            .as_i32_opt()
            .ok_or("failed to parse int")?,
        ma: sheet
            .value(row, 9)
            .as_i32_opt()
            .ok_or("failed to parse ma")?,
        q: sheet
            .value(row, 10)
            .as_i32_opt()
            .ok_or("failed to parse q")?,
        sit: sheet
            .value(row, 11)
            .as_i32_opt()
            .ok_or("failed to parse sit")?,
    })
}
