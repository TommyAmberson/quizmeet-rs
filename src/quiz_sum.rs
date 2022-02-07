use self::team::Team;
use self::quizzer::Quizzer;
use self::quiz::Quiz;

pub mod team;
pub mod quizzer;
pub mod quiz;
pub fn hello() -> String {
    let t = Team::default();
    t.print();
    let q = Quizzer::default();
    q.print();
    let qq = Quiz::default();
    qq.print();
    String::from("Hello, world!")
}
