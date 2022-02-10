// mod parse;
mod quiz_sum;

fn main() {
    // println!("{}", quiz_sum::hello());
    let mut sum = quiz_sum::Summary::new();
    sum.open_ods().unwrap();
    dbg!(&sum);
    // dbg!(sum.get_team_prelim());
}
