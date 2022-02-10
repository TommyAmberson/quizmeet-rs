use glob::glob;
use spreadsheet_ods::{error::OdsError, WorkBook};
use std::collections::HashMap;

#[derive(Debug)]
enum QuizType {
    Prelim(i32),
    Elim(String),
    Con((String, String)),
}
#[derive(Debug)]
struct Quiz {
    name: String,
    div: i32,
    quiz: QuizType,
}
impl Quiz {
    fn open(path_str: &str) -> Result<(Quiz, WorkBook), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path_str);
        let wb = spreadsheet_ods::read_ods(path)?;
        if wb.num_sheets() < 2 {
            return Err(Box::new(OdsError::Ods(String::from(
                "must have at least two sheets",
            ))));
        }
        Ok((
            Quiz {
                name: wb
                    .sheet(1)
                    .value(1, 1)
                    .as_str_opt()
                    .ok_or("failed to parse quiz name")?
                    .to_string(),
                div: wb
                    .sheet(0)
                    .value(2, 2)
                    .as_i32_opt()
                    .ok_or("failed to parse quiz div")?,
                quiz: {
                    let v = wb.sheet(0).value(2, 4);
                    v.as_i32_opt().map_or_else(
                        || {
                            v.as_str_opt()
                                .map(|q| -> Result<QuizType, Box<dyn std::error::Error>> {
                                    if q.len() > 1 && &q[..1] == "c" {
                                        Ok(QuizType::Con((q[..1].to_string(), q[1..].to_string())))
                                    } else {
                                        Ok(QuizType::Elim(q.to_string()))
                                    }
                                })
                                .ok_or("failed to parse quiz number")?
                        },
                        |q| Ok(QuizType::Prelim(q)),
                    )?
                    // match v {
                    //     spreadsheet_ods::Value::Text(t) => QuizType::Elim(t.to_string()),
                    //     spreadsheet_ods::Value::Number(n) => QuizType::Prelim(*n as i32),
                    //     _ => {
                    //         return Err(Box::new(OdsError::Ods(String::from(
                    //             "failed to parse quiz number",
                    //         ))))
                    //     }
                    // }
                    // .as_i32_opt()
                    // .as_str_opt()
                    // .ok_or("failed to parse quiz number")?.to_string(),
                },
            },
            wb,
        ))
    }
}
// fn open_wb(path_str: &str) -> Result<Quiz, Box<dyn std::error::Error>> {
//     let path = std::path::Path::new(path_str);
//     let wb = spreadsheet_ods::read_ods(path)?;
//     if wb.num_sheets() < 2 {
//         return Err(Box::new(OdsError::Ods(String::from(
//             "must have at least two sheets",
//         ))));
//     }
//     Ok(wb)
// }

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
pub struct Summary {
    teams: HashMap<String, Vec<TeamEntry>>,
    quizzers: HashMap<String, Vec<QuizzerEntry>>,
    quizes: Vec<Quiz>,
}

impl Summary {
    pub fn new() -> Self {
        Summary {
            teams: HashMap::new(),
            quizzers: HashMap::new(),
            quizes: Vec::new(),
        }
    }
    fn insert(&mut self, entry: Entry) {
        match entry {
            Entry::Team(t) => match self.teams.get_mut(&t.name.to_string()) {
                Some(v) => {
                    v.push(t);
                }
                None => {
                    self.teams.insert(t.name.to_string(), vec![t]);
                }
            },
            Entry::Quizzer(q) => match self.quizzers.get_mut(&q.name.to_string()) {
                Some(v) => {
                    v.push(q);
                }
                None => {
                    self.quizzers.insert(q.name.to_string(), vec![q]);
                }
            },
        }
    }

    pub fn open(&mut self, path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (quiz, wb) = Quiz::open(path_str)?;
        self.quizes.push(quiz);
        self.parse(&wb)?;
        Ok(())
    }
    pub fn open_test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.open("tests/D1Q1.ods")
    }

    fn parse(&mut self, wb: &WorkBook) -> Result<(), Box<dyn std::error::Error>> {
        // let mut entries: Vec<Entry> = Vec::new();
        for row in 1..4 {
            let team = match TeamEntry::get_from_row(wb, row) {
                Ok(team) => team,
                Err(_) => continue,
            };
            self.insert(Entry::Team(team));
        }
        for row in 6..21 {
            let quizzer = match QuizzerEntry::get_from_row(wb, row) {
                Ok(quizzer) => quizzer,
                Err(_) => continue,
            };
            self.insert(Entry::Quizzer(quizzer));
        }
        // Ok(entries)
        Ok(())
    }

    // pub fn open(&self, path: &str) {
    //     let wb = open(path).unwrap();
    //     dbg!(parse(&wb, &mut self).unwrap());
    // }
    // pub fn get_team_prelim(&self) -> Vec<&str> {
    //     let l = self
    //         .entries
    //         .iter()
    //         .filter_map(|q| -> Option<&str> {
    //             match q {
    //                 Entry::Quizzer(_) => None,
    //                 Entry::Team(t) => Some(&t.name),
    //             }
    //         })
    //         .collect();
    //     l
    // }
}
