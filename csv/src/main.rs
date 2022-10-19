//tutorial-setup-01.rs
// Import the standard library's I/O module so we can read from stdin.
use std::{
    error::Error,
    io::{self, Read},
};

use csv_parse::types::{
    board::board::Board,
    quizzer::summary::{QuestionTypeCount, QuizzerSummaryRecord},
    team::summary::TeamSummaryRecord,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Record {
    // #[serde(flatten)]
    Quizzer(QuizzerSummaryRecord),
    Team(TeamSummaryRecord),
}

// The `main` function is where your program starts executing.
fn main() {
    println!(
        "{}",
        serde_json::to_string(&Record::Quizzer(QuizzerSummaryRecord {
            quizzer: "some quizzer".to_string(),
            team: "some quizzer".to_string(),
            quiz: "some quizzer".to_string(),
            points: 0,
            errors: 0,
            jumps: 0,
            qtc: QuestionTypeCount {
                reference: 0,
                finish_the_verse: 0,
                straightforward: 0,
                multiple_answer: 0,
                quote: 0,
                situation: 0
            }
        }))
        .unwrap()
    );

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    // let input2 = input.clone();
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::BufReader::new(input.as_slice()));
    match {
        if rdr
            .headers()
            .map(|rec| rec[0] == "Quizzer".to_owned())
            .unwrap_or(false)
        {
            println!("Quizzer");
            des_print::<QuizzerSummaryRecord, _>(&mut rdr)
        } else if rdr
            .headers()
            .map(|rec| rec[0] == "Team".to_owned())
            .unwrap_or(false)
        {
            println!("Team");
            des_print::<TeamSummaryRecord, _>(&mut rdr)
        } else {
            println!("Board");
            des_json(input.clone());
            Ok(())
        }
    } {
        Ok(()) => {
            println!("OK");
        }
        Err(err) => {
            println!("Board because {}", err);
            des_json(input);
        }
    }
}
pub fn des_json(input: Vec<u8>) {
    println!("input: {}", String::from_utf8(input.clone()).unwrap());
    let s = serde_json::from_slice(input.as_slice());
    let res: Board = s.unwrap();
    println!("board res: {:?}", res);
}
pub fn des_print<T, R>(rdr: &mut csv::Reader<R>) -> Result<(), Box<dyn Error>>
where
    T: for<'de> Deserialize<'de> + std::fmt::Debug,
    R: std::io::Read,
{
    // Loop over each record.
    for result in rdr.deserialize() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record: T = result?;
        // Print a debug version of the record.
        println!("{:?}", record);
    }
    Ok(())
}
