use clap::Parser;
use quizmeet_rs::io::*;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    glob: Option<String>,

    #[clap(short, long)]
    regex: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let g = args.glob.unwrap_or(String::from("json/*.json"));
    // let g = String::from("json/*.json");
    let re = Regex::new(&args.regex.unwrap_or(String::from(r"^json/D(?P<d>\dC?)Q(?P<q>(\d|\w)+).json$"))).unwrap();
    dbg!(&re);
    from_glob(&g, |e| {
        if re.is_match(e.to_str().unwrap()) {
            dbg!(&e);
        }
        // dbg!(&e);
        Ok(())
    })
    .unwrap();
}
