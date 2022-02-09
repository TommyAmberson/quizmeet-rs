#[derive(Debug)]
pub struct TeamEntry {
    name: String,
    quiz: String,
    place: f64,
    score: i32,
    points: i32,
    errors: i32,
}

impl TeamEntry {
    pub fn new(
        name: String,
        quiz: String,
        place: f64,
        score: i32,
        points: i32,
        errors: i32,
    ) -> Self {
        TeamEntry {
            name,
            quiz,
            place,
            score,
            points,
            errors,
        }
    }
}

impl Default for TeamEntry {
    fn default() -> Self {
        TeamEntry {
            name: String::from("test team name"),
            quiz: String::from("test quiz name"),
            place: 0.0,
            score: 0,
            points: 0,
            errors: 0,
        }
    }
}
