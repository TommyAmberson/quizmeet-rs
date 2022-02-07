mod parse;
mod quiz_sum;

fn main() {
    // println!("{}", quiz_sum::hello());
    let wb = parse::open("tests/D1Q1.ods").unwrap();
    dbg!(parse::parse(&wb).unwrap());
}
