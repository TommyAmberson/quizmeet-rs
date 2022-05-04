use clap::Parser;
use quizmeet_rs::io::*;

#[derive(Parser)]
struct Cli {
    g: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let g = args.g.unwrap_or(String::from("ods/*.ods"));
    from_glob(&g, translate).unwrap();
}
