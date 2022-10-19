use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TeamSummaryRecord {
    team: String,
    quiz: String,
    place: i32,
    score: i32,
    points: i32,
    errors: i32,
}
