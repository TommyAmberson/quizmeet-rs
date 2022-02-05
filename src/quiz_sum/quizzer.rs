use self::super::team::Team;

#[derive(Debug)]
pub struct Quizzer {
    name: String,
    team: Team,
    quiz: i32,
    points: i32,
    errors: i32,
    jumps: i32,
    refer: i32,
    ftv: i32,
    int: i32,
    ma: i32,
    q: i32,
    sit: i32,
}

impl Quizzer {
    pub fn new() -> Self {
        Quizzer {
            name: String::from("test name"),
            team: Team::new(),
            quiz: 0,
            points: 0,
            errors: 0,
            jumps: 0,
            refer: 0,
            ftv: 0,
            int: 0,
            ma: 0,
            q: 0,
            sit: 0,
        }
    }
    pub fn print(&self) {
        println!("{:?}", self);
    }
}
