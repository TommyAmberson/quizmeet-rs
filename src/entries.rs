use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizEntry {
    pub team_entries: Vec<TeamEntry>,
    pub quizzer_entries: Vec<QuizzerEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamEntry {
    pub name: String,
    pub quiz: String,
    pub place: f64,
    pub score: i32,
    pub points: i32,
    pub errors: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizzerEntry {
    pub name: String,
    pub team: String,
    pub quiz: String,
    pub points: i32,
    pub errors: i32,
    pub jumps: i32,
    pub refer: i32,
    pub ftv: i32,
    pub int: i32,
    pub ma: i32,
    pub q: i32,
    pub sit: i32,
}
