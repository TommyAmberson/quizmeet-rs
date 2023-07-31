use serde::{Deserialize, Serialize};

use crate::quiz::TeamEntry;
use crate::stats::error::StatsError;
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TeamStats {
    pub name: String,

    pub score: i32,
    pub points: i32,
    pub errors: i32,

    pub quizzes: Vec<TeamEntry>,
}

impl Stats<TeamEntry> for TeamStats {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, entry: TeamEntry) -> Result<(), StatsError> {
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
        self.score += entry.score;
        self.points += entry.points;
        self.errors += entry.errors;
        self.quizzes.push(entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        self.score as f32 / self.quizzes.len() as f32
    }
}

impl From<TeamEntry> for TeamStats {
    fn from(value: TeamEntry) -> Self {
        Self {
            name: value.name.clone(),

            score: value.score,
            points: value.points,
            errors: value.errors,

            quizzes: vec![value],
        }
    }
}
