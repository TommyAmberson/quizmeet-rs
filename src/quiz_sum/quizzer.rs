// use self::super::team::TeamEntry;

#[derive(Debug)]
pub struct QuizzerEntry {
    name: String,
    team: String,
    quiz: String,
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

impl QuizzerEntry {
    pub fn new(
        name: String,
        team: String,
        quiz: String,
        points: i32,
        errors: i32,
        jumps: i32,
        refer: i32,
        ftv: i32,
        int: i32,
        ma: i32,
        q: i32,
        sit: i32,
    ) -> Self {
        QuizzerEntry {
            name,
            team,
            quiz,
            points,
            errors,
            jumps,
            refer,
            ftv,
            int,
            ma,
            q,
            sit,
        }
    }
}
impl Default for QuizzerEntry {
    fn default() -> Self {
        QuizzerEntry {
            name: String::from("test quizzer name"),
            team: String::from("test team name"),
            quiz: String::from("test quiz name"),
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
}
