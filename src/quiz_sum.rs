use spreadsheet_ods::{error::OdsError, WorkBook};

fn open(path_str: &str) -> Result<WorkBook, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(path_str);
    let wb = spreadsheet_ods::read_ods(path)?;
    if wb.num_sheets() < 2 {
        return Err(Box::new(OdsError::Ods(String::from(
            "must have at least two sheets",
        ))));
    }
    Ok(wb)
}
fn parse(wb: &WorkBook, entries: &mut Vec<Entry>) -> Result<(), Box<dyn std::error::Error>> {
    // let mut entries: Vec<Entry> = Vec::new();
    for row in 1..4 {
        let team = match TeamEntry::get_from_row(wb, row) {
            Ok(team) => team,
            Err(_) => continue,
        };
        entries.push(Entry::Team(team));
    }
    for row in 6..21 {
        let quizzer = match QuizzerEntry::get_from_row(wb, row) {
            Ok(quizzer) => quizzer,
            Err(_) => continue,
        };
        entries.push(Entry::Quizzer(quizzer));
    }
    // Ok(entries)
    Ok(())
}

#[derive(Debug)]
enum Entry {
    Team(TeamEntry),
    Quizzer(QuizzerEntry),
}

#[derive(Debug)]
struct TeamEntry {
    name: String,
    quiz: String,
    place: f64,
    score: i32,
    points: i32,
    errors: i32,
}

impl TeamEntry {
    fn get_from_row(wb: &WorkBook, row: u32) -> Result<TeamEntry, Box<dyn std::error::Error>> {
        let sheet = wb.sheet(1);
        Ok(TeamEntry {
            name: String::from(
                sheet
                    .value(row, 0)
                    .as_str_opt()
                    .ok_or("failed to parse name")?,
            ),
            quiz: String::from(
                sheet
                    .value(row, 1)
                    .as_str_opt()
                    .ok_or("failed to parse quiz")?,
            ),
            place: sheet
                .value(row, 2)
                .as_f64_opt()
                .ok_or("failed to parse place")?,
            score: sheet
                .value(row, 3)
                .as_i32_opt()
                .ok_or("failed to parse score")?,
            points: sheet
                .value(row, 4)
                .as_i32_opt()
                .ok_or("failed to parse points")?,
            errors: sheet
                .value(row, 5)
                .as_i32_opt()
                .ok_or("failed to parse errors")?,
        })
    }
}

#[derive(Debug)]
struct QuizzerEntry {
    name: String,
    team: String,
    quiz: String,
    points: i32,
    errors: i32,
    jumps: i32,
    refer: i32,
    ftv: i32,
    int: i32,
    ma: i32,
    q: i32,
    sit: i32,
}

impl QuizzerEntry {
    fn get_from_row(wb: &WorkBook, row: u32) -> Result<QuizzerEntry, Box<dyn std::error::Error>> {
        let sheet = wb.sheet(1);
        Ok(QuizzerEntry {
            name: String::from(
                sheet
                    .value(row, 0)
                    .as_str_opt()
                    .ok_or("failed to parse name")?,
            ),
            team: String::from(
                sheet
                    .value(row, 1)
                    .as_str_opt()
                    .ok_or("failed to parse team")?,
            ),
            quiz: String::from(
                sheet
                    .value(row, 2)
                    .as_str_opt()
                    .ok_or("failed to parse quiz")?,
            ),
            points: sheet
                .value(row, 3)
                .as_i32_opt()
                .ok_or("failed to parse points")?,
            errors: sheet
                .value(row, 4)
                .as_i32_opt()
                .ok_or("failed to parse errors")?,
            jumps: sheet
                .value(row, 5)
                .as_i32_opt()
                .ok_or("failed to parse jumps")?,
            refer: sheet
                .value(row, 6)
                .as_i32_opt()
                .ok_or("failed to parse refer")?,
            ftv: sheet
                .value(row, 7)
                .as_i32_opt()
                .ok_or("failed to parse ftv")?,
            int: sheet
                .value(row, 8)
                .as_i32_opt()
                .ok_or("failed to parse int")?,
            ma: sheet
                .value(row, 9)
                .as_i32_opt()
                .ok_or("failed to parse ma")?,
            q: sheet
                .value(row, 10)
                .as_i32_opt()
                .ok_or("failed to parse q")?,
            sit: sheet
                .value(row, 11)
                .as_i32_opt()
                .ok_or("failed to parse sit")?,
        })
    }
}

#[derive(Debug)]
struct QuizEntry {
    name: String,
    teams: Vec<TeamEntry>,
    quizzers: Vec<QuizzerEntry>,
}

pub fn open_test() {
    let wb = open("tests/D1Q1.ods").unwrap();
    dbg!(parse(&wb).unwrap());
}

#[derive(Debug)]
struct QuizSummary {
    entries: Vec<Entry>,
}

impl QuizSummary {
    pub fn new() -> Self {
        let wb = open("tests/D1Q1.ods").unwrap();
        dbg!(parse(&wb).unwrap());
        let entries = parse(&wb).unwrap();
        QuizSummary { entries }
    }
    // pub fn get_team_prelim(&self) -> Vec<&str> {
    //     self.quizzes.iter().filter(|q|)
    //     let list: Vec<Team> = Vec::new();
    //     for quiz in self.quizzes {
    //         list.push(quiz);
    //     }
    //     let l = list.iter().map(|q| {
    //         q
    //     }).collect();
    // }
}
