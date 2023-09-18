use std::collections::HashMap;

use anyhow::{bail, Result};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

use crate::name::{QuizName, TeamName};
use crate::quiz::{Team, TeamEntry};
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs, PartialEq)]
pub struct TeamStats {
    pub name: TeamName,
    pub quizzes: HashMap<QuizName, TeamEntry>,
}

impl Stats<TeamEntry> for TeamStats {
    fn update(&mut self, entry: TeamEntry) -> Result<bool> {
        if entry.name != self.name {
            bail!("Name must be the same for stats: '{self:?}' and entry: '{entry:?}'");
        }

        if self
            .quizzes
            .get(&entry.quiz)
            .map(|e| e == &entry)
            .unwrap_or(false)
        {
            return Ok(false);
        }

        self.quizzes.insert(entry.quiz.clone(), entry);
        Ok(true)
    }

    fn avg(&self) -> f32 {
        self.points() as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        self.score() as f32 / self.quizzes.len() as f32
    }
}

impl TeamStats {
    pub fn new(name: TeamName) -> Self {
        Self {
            name,
            quizzes: HashMap::new(),
        }
    }
}

impl Team for TeamStats {
    fn score(&self) -> i32 {
        self.quizzes.values().map(Team::score).sum()
    }
    fn points(&self) -> i32 {
        self.quizzes.values().map(Team::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.values().map(Team::errors).sum()
    }
}
