use self::super::quizzer::Quizzer;
use self::super::team::Team;

#[derive(Debug)]
pub struct Quiz {
    name: String,
    teams: Vec<Team>,
    quizzers: Vec<Quizzer>,
}

impl Quiz {
    pub fn new(name: String, teams: Vec<Team>, quizzers: Vec<Quizzer>) -> Self {
        Quiz {
            name,
            teams,
            quizzers,
        }
    }
}
impl Default for Quiz {
    fn default() -> Self {
        Quiz {
            name: String::from("test name"),
            teams: vec![Team::default(), Team::default(), Team::default()],
            quizzers: vec![
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
                Quizzer::default(),
            ],
        }
    }
}
