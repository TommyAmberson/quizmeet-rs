use crate::{entries::*, io::*};

pub fn open_json(g: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let g = g.unwrap_or("json/*.json");
    let mut team_entries: Vec<TeamEntry> = Vec::new();
    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    let r = from_glob(&g, |entry| {
        let result = read(entry.as_path())?;
        // dbg!(&result);
        team_entries.extend(result.0);
        quizzer_entries.extend(result.1);
        Ok(())
    })?;
    dbg!(&r);
    let team_sums = group_and_sum(team_entries)?;
    dbg!(team_sums);
    let quizzer_sums = group_and_sum(quizzer_entries)?;
    dbg!(quizzer_sums);
    Ok(())
}
