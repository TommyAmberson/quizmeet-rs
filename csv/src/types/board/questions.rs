use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum QuestionNum {
    Q1, // {
    //     team: i32,
    //     quizzer: i32,
    // }
    Q2, // {
    //     team: i32,
    //     quizzer: i32,
    // }
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
    Q8,
    Q9,
    Q10,
    Q11,
    Q12,
    Q13,
    Q14,
    Q15,
    Q16,
    Q16A,
    Q16B,
    Q17,
    Q17A,
    Q17B,
    Q18,
    Q18A,
    Q18B,
    Q19A,
    Q19B,
    Q20,
    Q20A,
    Q20B,
    Q21,
    Q21A,
    Q21B,
    Q22,
    Q22A,
    Q22B,
    Q23,
    Q23A,
    Q23B,
    Q24,
    Q24A,
    Q24B,
    Q25,
    Q25A,
    Q25B,
    Q26,
    Q26A,
    Q26B,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "PascalCase")]
pub struct Question {
    team: u8,
    quizzer: u8,
    answer: Anwser,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Anwser {
    #[serde(rename = "c")]
    Correct,
    #[serde(rename = "e")]
    Error,
    #[serde(rename = "b")]
    Bonus,
    #[serde(rename = "mb")]
    MissedBonus,
}
