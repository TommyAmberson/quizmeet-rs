#[derive(Debug)]
pub struct Team {
    name: String,
    quiz: i32,
    place: f32,
    score: i32,
    points: i32,
    errors: i32,
}

impl Team {
    pub fn new() -> Self {
        Team {
            name: String::from("test name"),
            quiz: 0,
            place: 0.0,
            score: 0,
            points: 0,
            errors: 0,
        }
    }
    pub fn print(&self) {
        println!("{:?}", self);
    }
}
