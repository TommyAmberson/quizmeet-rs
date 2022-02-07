use crate::parse::QuizFile;

mod parse;
mod quiz_sum;

fn main() {
    println!("{}", quiz_sum::hello());
    let qf = QuizFile::open("tests/D1Q1.ods").unwrap();
    dbg!(qf.parse().unwrap());
}
