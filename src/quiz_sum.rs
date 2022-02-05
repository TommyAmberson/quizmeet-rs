use self::team::Team;
use self::quizzer::Quizzer;

mod team;
mod quizzer;
pub fn hello() -> String {
    let t = Team::new();
    t.print();
    let q = Quizzer::new();
    q.print();
    String::from("Hello, world!")
}
