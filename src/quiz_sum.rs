use crate::quiz_sum::parse::parse;

use self::quiz::Quiz;
use self::quizzer::QuizzerEntry;
use self::team::TeamEntry;

pub mod parse;
pub mod quiz;
pub mod quizzer;
pub mod team;

pub fn open() {
    let wb = parse::open("tests/D1Q1.ods").unwrap();
    dbg!(parse::parse(&wb).unwrap());
}

#[derive(Debug)]
struct QuizSummary {
    quizzes: Vec<Quiz>,
}

impl QuizSummary {
    pub fn new() -> Self {
        let wb = parse::open("tests/D1Q1.ods").unwrap();
        dbg!(parse::parse(&wb).unwrap());
        let quizzes = vec![parse(&wb).unwrap()];
        QuizSummary { quizzes }
    }
    pub fn get_team_prelim() -> Vec<TeamEntry> {
        unimplemented!();
    }
    // fn get_order<T>(&self) -> Vec<T> {
    //
    // }
}
