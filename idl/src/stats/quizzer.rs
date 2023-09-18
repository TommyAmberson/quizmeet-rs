use std::collections::HashMap;

use anyhow::{bail, Result};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

use crate::name::{QuizName, QuizzerName};
use crate::quiz::{Quizzer, QuizzerEntry};
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs, PartialEq)]
pub struct QuizzerStats {
    pub name: QuizzerName,
    pub quizzes: HashMap<QuizName, QuizzerEntry>,
}

impl Stats<QuizzerEntry> for QuizzerStats {
    fn update(&mut self, entry: QuizzerEntry) -> Result<bool> {
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
        if self.errors() > 0 {
            1. / self.errors() as f32
        } else {
            10.
        }
    }
}

impl QuizzerStats {
    pub fn new(name: QuizzerName) -> Self {
        Self {
            name,
            quizzes: HashMap::new(),
        }
    }
}

impl Quizzer for QuizzerStats {
    fn points(&self) -> i32 {
        self.quizzes.values().map(Quizzer::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.values().map(Quizzer::errors).sum()
    }
    fn jumps(&self) -> i32 {
        self.quizzes.values().map(Quizzer::jumps).sum()
    }
    fn refer(&self) -> i32 {
        self.quizzes.values().map(Quizzer::refer).sum()
    }
    fn ftv(&self) -> i32 {
        self.quizzes.values().map(Quizzer::ftv).sum()
    }
    fn int(&self) -> i32 {
        self.quizzes.values().map(Quizzer::int).sum()
    }
    fn ma(&self) -> i32 {
        self.quizzes.values().map(Quizzer::ma).sum()
    }
    fn q(&self) -> i32 {
        self.quizzes.values().map(Quizzer::q).sum()
    }
    fn sit(&self) -> i32 {
        self.quizzes.values().map(Quizzer::sit).sum()
    }
}
