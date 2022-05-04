use clap::Parser;
use quizmeet_rs::{entries::*, io::*};

#[derive(Parser)]
struct Cli {
    g: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let g = args.g.unwrap_or(String::from("json/*.json"));
    let mut team_entries: Vec<TeamEntry> = Vec::new();
    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    let r = from_glob(&g, |entry| {
        let result = read(entry.as_path())?;
        // dbg!(&result);
        team_entries.extend(result.0);
        quizzer_entries.extend(result.1);
        Ok(())
    });
    dbg!(&r);
    r.unwrap();
    // println!("team_entries: {:?}\nquizzer_entries: {:?}", team_entries, quizzer_entries);
}
