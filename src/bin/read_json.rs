use clap::Parser;
use quizmeet_rs::stats::open_json;

#[derive(Parser)]
struct Cli {
    g: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let r = open_json(args.g, None);
    dbg!(&r);
    let (team_sums, quizzer_sums) = r.unwrap();
    dbg!(team_sums);
    dbg!(quizzer_sums);
    // println!("team_entries: {:?}\nquizzer_entries: {:?}", team_entries, quizzer_entries);
}
