use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QuizzerSummaryRecord {
    pub quizzer: String,
    pub team: String,
    pub quiz: String,
    pub points: i32,
    pub errors: i32,
    pub jumps: i32,
    #[serde(flatten)]
    pub qtc: QuestionTypeCount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionTypeCount {
    #[serde(rename = "REF")]
    pub reference: i32,
    #[serde(rename = "FTV")]
    pub finish_the_verse: i32,
    #[serde(rename = "INT")]
    pub straightforward: i32,
    #[serde(rename = "MA")]
    pub multiple_answer: i32,
    #[serde(rename = "Q")]
    pub quote: i32,
    #[serde(rename = "SIT")]
    pub situation: i32,
}
