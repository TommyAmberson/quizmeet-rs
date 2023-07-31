use serde::{Deserialize, Serialize};

use crate::quiz::QuizzerEntry;
use crate::stats::error::StatsError;
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuizzerStats {
    pub name: String,
    pub team: String,

    pub points: i32,
    pub errors: i32,
    pub jumps: i32,
    pub refer: i32,
    pub ftv: i32,
    pub int: i32,
    pub ma: i32,
    pub q: i32,
    pub sit: i32,

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

        self.points += entry.points;
        self.errors += entry.errors;
        self.jumps += entry.jumps;
        self.refer += entry.refer;
        self.ftv += entry.ftv;
        self.int += entry.int;
        self.ma += entry.ma;
        self.q += entry.q;
        self.sit += entry.sit;

        self.quizzes.push(entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        if self.errors > 0 {
            1. / self.errors as f32
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

            points: value.points,
            errors: value.errors,
            jumps: value.jumps,
            refer: value.refer,
            ftv: value.ftv,
            int: value.int,
            ma: value.ma,
            q: value.q,
            sit: value.sit,

            quizzes: vec![value],
        }
    }
}
