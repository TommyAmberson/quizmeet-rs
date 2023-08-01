use serde::{Deserialize, Serialize};

use crate::name::{QuizName, QuizzerName, TeamName};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Quiz {
    pub team_entries: Vec<TeamEntry>,
    pub quizzer_entries: Vec<QuizzerEntry>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TeamEntry {
    pub name: TeamName,
    pub quiz: QuizName,
    pub place: f64,
    pub score: i32,
    pub points: i32,
    pub errors: i32,
}

pub trait Team {
    fn score(&self) -> i32;
    fn points(&self) -> i32;
    fn errors(&self) -> i32;
}
impl Team for TeamEntry {
    fn score(&self) -> i32 {
        self.score
    }
    fn points(&self) -> i32 {
        self.points
    }
    fn errors(&self) -> i32 {
        self.errors
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuizzerEntry {
    pub name: QuizzerName,
    pub team: TeamName,
    pub quiz: QuizName,
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
pub trait Quizzer {
    fn points(&self) -> i32;
    fn errors(&self) -> i32;
    fn jumps(&self) -> i32;
    fn refer(&self) -> i32;
    fn ftv(&self) -> i32;
    fn int(&self) -> i32;
    fn ma(&self) -> i32;
    fn q(&self) -> i32;
    fn sit(&self) -> i32;
}
impl Quizzer for QuizzerEntry {
    fn points(&self) -> i32 {
        self.points
    }
    fn errors(&self) -> i32 {
        self.errors
    }
    fn jumps(&self) -> i32 {
        self.jumps
    }
    fn refer(&self) -> i32 {
        self.refer
    }
    fn ftv(&self) -> i32 {
        self.ftv
    }
    fn int(&self) -> i32 {
        self.int
    }
    fn ma(&self) -> i32 {
        self.ma
    }
    fn q(&self) -> i32 {
        self.q
    }
    fn sit(&self) -> i32 {
        self.sit
    }
}
