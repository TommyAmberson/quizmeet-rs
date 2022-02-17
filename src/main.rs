// mod parse;
mod quiz_sum;
use quiz_sum::QuizType;

fn main() {
    // println!("{}", quiz_sum::hello());
    let mut sum = quiz_sum::Summary::new();
    sum.open_ods().unwrap();
    dbg!(&sum);
    // dbg!(sum.get_team_prelims(1));
    dbg!(sum.get_team_order(|q| q.div == 1 && matches!(q.quiz, QuizType::Preliminary(_))));
}
