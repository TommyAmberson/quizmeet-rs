use serde::{Deserialize, Serialize};

use crate::quiz::{Team, TeamEntry};
use crate::stats::error::StatsError;
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TeamStats {
    pub name: String,
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
        self.quizzes.push(entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points() as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        self.score() as f32 / self.quizzes.len() as f32
    }
}

impl From<TeamEntry> for TeamStats {
    fn from(value: TeamEntry) -> Self {
        Self {
            name: value.name.clone(),
            quizzes: vec![value],
        }
    }
}

impl Team for TeamStats {
    fn score(&self) -> i32 {
        self.quizzes.iter().map(Team::score).sum()
    }
    fn points(&self) -> i32 {
        self.quizzes.iter().map(Team::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.iter().map(Team::errors).sum()
    }
}
