use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::questions::{Question, QuestionNum};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    // questions: Vec<QuestionNum>,
    questions: HashMap<QuestionNum, Question>,
    // questions: serde_json::Value,
}
