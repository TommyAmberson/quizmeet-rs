// mod parse;
mod quiz_sum;

fn main() {
    // println!("{}", quiz_sum::hello());
    let sum = quiz_sum::QuizSummary::new();
    dbg!(&sum);
    dbg!(sum.get_team_prelim());
}
