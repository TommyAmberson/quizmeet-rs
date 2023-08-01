use serde::{Deserialize, Serialize};

use crate::quiz::{Quizzer, QuizzerEntry};
use crate::stats::error::StatsError;
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuizzerStats {
    pub name: String,
    pub team: String,
    pub quizzes: Vec<QuizzerEntry>,
}

impl Stats<QuizzerEntry> for QuizzerStats {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, entry: QuizzerEntry) -> Result<(), StatsError> {
        if entry.name != self.name {
            return Err(StatsError::BadName {
                stats: self.name.clone(),
                entry: format!("{entry:?}"),
            });
        }
        if self.quizzes.contains(&entry) {
            return Err(StatsError::DuplicateEntry {
                stats: self.name.clone(),
                entry: format!("{entry:?}"),
            });
        }

        self.quizzes.push(entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points() as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        if self.errors() > 0 {
            1. / self.errors() as f32
        } else {
            10.
        }
    }
}

impl From<QuizzerEntry> for QuizzerStats {
    fn from(value: QuizzerEntry) -> Self {
        Self {
            name: value.name.clone(),
            team: value.team.clone(),
            quizzes: vec![value],
        }
    }
}

impl Quizzer for QuizzerStats {
    fn points(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::errors).sum()
    }
    fn jumps(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::jumps).sum()
    }
    fn refer(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::refer).sum()
    }
    fn ftv(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::ftv).sum()
    }
    fn int(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::int).sum()
    }
    fn ma(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::ma).sum()
    }
    fn q(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::q).sum()
    }
    fn sit(&self) -> i32 {
        self.quizzes.iter().map(Quizzer::sit).sum()
    }
}
