#[derive(Debug)]
pub struct Team {
    name: String,
    quiz: String,
    place: f64,
    score: i32,
    points: i32,
    errors: i32,
}

impl Team {
    pub fn new(
        name: String,
        quiz: String,
        place: f64,
        score: i32,
        points: i32,
        errors: i32,
    ) -> Self {
        Team {
            name,
            quiz,
            place,
            score,
            points,
            errors,
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team {
            name: String::from("test team name"),
            quiz: String::from("test quiz name"),
            place: 0.0,
            score: 0,
            points: 0,
            errors: 0,
        }
    }
}
