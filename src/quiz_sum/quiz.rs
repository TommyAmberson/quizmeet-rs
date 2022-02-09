use self::super::quizzer::QuizzerEntry;
use self::super::team::TeamEntry;

#[derive(Debug)]
pub struct Quiz {
    name: String,
    teams: Vec<TeamEntry>,
    quizzers: Vec<QuizzerEntry>,
}

impl Quiz {
    pub fn new(name: String, teams: Vec<TeamEntry>, quizzers: Vec<QuizzerEntry>) -> Self {
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
            teams: vec![TeamEntry::default(), TeamEntry::default(), TeamEntry::default()],
            quizzers: vec![
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
                QuizzerEntry::default(),
            ],
        }
    }
}
