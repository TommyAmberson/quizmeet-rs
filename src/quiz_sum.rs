use self::quiz::Quiz;
use self::quizzer::Quizzer;
use self::team::Team;

pub mod quiz;
pub mod quizzer;
pub mod team;
pub fn hello() -> String {
    let t = Team::default();
    dbg!(t);
    let q = Quizzer::default();
    dbg!(q);
    let qq = Quiz::default();
    dbg!(qq);
    String::from("Hello, world!")
}
