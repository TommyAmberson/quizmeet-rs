use glob::glob;
use itertools::Itertools;
use spreadsheet_ods::{error::OdsError, WorkBook};
use std::rc::Rc;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub enum QuizType {
    Preliminary(i32),
    Elimination(String),
    Consolation((String, String)),
}
#[derive(Debug)]
pub struct Quiz {
    name: String,
    pub div: i32,
    pub quiz: QuizType,
}
impl Quiz {
    fn open(path: &Path) -> Result<(Quiz, WorkBook), Box<dyn std::error::Error>> {
        // let path = std::path::Path::new(path_str);
        dbg!(path.display());
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
                                        Ok(QuizType::Consolation((
                                            q[..1].to_string(),
                                            q[1..].to_string(),
                                        )))
                                    } else {
                                        Ok(QuizType::Elimination(q.to_string()))
                                    }
                                })
                                .ok_or(
                                    "failed to parse quiz number: ".to_owned()
                                        + (path.to_str().unwrap_or("invalid path")),
                                )?
                        },
                        |q| Ok(QuizType::Preliminary(q)),
                    )?
                },
            },
            wb,
        ))
    }
}

#[derive(Debug)]
enum Entry {
    Team(TeamEntry),
    Quizzer(QuizzerEntry),
}

#[derive(Debug)]
struct TeamEntry {
    name: String,
    quiz: Rc<Quiz>,
    place: f64,
    score: i32,
    points: i32,
    errors: i32,
}

impl TeamEntry {
    fn get_from_row(
        wb: &WorkBook,
        row: u32,
        quiz: &Rc<Quiz>,
    ) -> Result<TeamEntry, Box<dyn std::error::Error>> {
        let sheet = wb.sheet(1);
        Ok(TeamEntry {
            name: String::from(
                sheet
                    .value(row, 0)
                    .as_str_opt()
                    .ok_or("failed to parse name")?,
            ),
            quiz: Rc::clone(quiz),
            // quiz: String::from(
            //     sheet
            //         .value(row, 1)
            //         .as_str_opt()
            //         .ok_or("failed to parse quiz")?,
            // ),
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
    quiz: Rc<Quiz>,
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
    fn get_from_row(
        wb: &WorkBook,
        row: u32,
        quiz: &Rc<Quiz>,
    ) -> Result<QuizzerEntry, Box<dyn std::error::Error>> {
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
            quiz: Rc::clone(quiz),
            // quiz: String::from(
            //     sheet
            //         .value(row, 2)
            //         .as_str_opt()
            //         .ok_or("failed to parse quiz")?,
            // ),
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
    quizzes: HashMap<String, Rc<Quiz>>,
}

impl Summary {
    pub fn new() -> Self {
        Summary {
            teams: HashMap::new(),
            quizzers: HashMap::new(),
            quizzes: HashMap::new(),
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

    pub fn open(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let (quiz, wb) = Quiz::open(path)?;
        let quiz = Rc::new(quiz);
        self.parse(&wb, &quiz)?;
        self.quizzes.insert(quiz.name.to_string(), quiz);
        Ok(())
    }
    pub fn open_test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.open(Path::new("ods/D1Q1.ods"))
    }
    pub fn open_ods(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in glob("ods/*.ods")?.filter_map(Result::ok) {
            self.open(entry.as_path())?;
        }
        Ok(())
    }

    fn parse(&mut self, wb: &WorkBook, quiz: &Rc<Quiz>) -> Result<(), Box<dyn std::error::Error>> {
        // let mut entries: Vec<Entry> = Vec::new();
        for row in 1..4 {
            let team = match TeamEntry::get_from_row(wb, row, quiz) {
                Ok(team) => team,
                Err(_) => continue,
            };
            self.insert(Entry::Team(team));
        }
        for row in 6..21 {
            let quizzer = match QuizzerEntry::get_from_row(wb, row, quiz) {
                Ok(quizzer) => quizzer,
                Err(_) => continue,
            };
            self.insert(Entry::Quizzer(quizzer));
        }
        // Ok(entries)
        Ok(())
    }

    pub fn get_team_prelims(&self, div: i32) -> Vec<&str> {
        self.get_team_order(|q| q.div == div && matches!(q.quiz, QuizType::Preliminary(_)))
    }
    // pub fn get_team_order
    pub fn get_team_order<F>(&self, quiz_mask: F) -> Vec<&str>
    where
        // Ft: Fn(&(&String, &Vec<TeamEntry>)) -> bool,
        F: Fn(&Quiz) -> bool,
    {
        self.teams
            .iter()
            // .filter(team_mask)
            .map(|(name, quizzes)| {
                (
                    name,
                    quizzes
                        .iter()
                        .filter(|t| quiz_mask(&t.quiz))
                        .fold(0., |total, t| {
                            total + (t.points as f32) + (t.score as f32) / 10000.
                        }),
                )
            })
            .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(name, _points)| -> &str { name })
            .collect()
    }
    pub fn get_quizzer_order<F>(&self, quiz_mask: F) -> Vec<&str>
    where
        // Ft: Fn(&(&String, &Vec<TeamEntry>)) -> bool,
        F: Fn(&Quiz) -> bool,
    {
        self.quizzers
            .iter()
            // .filter(team_mask)
            .map(|(name, quizzes)| {
                let (points, errors) = quizzes
                    .iter()
                    .filter(|q| quiz_mask(&q.quiz))
                    .fold((0, 0), |(points, errors), q| {
                        (points + q.points, errors + q.errors)
                    });
                let average = points as f32 / quizzes.len() as f32;
                let error_bonus = if errors > 0 {
                    1. / (errors as f32 * 1000.)
                } else {
                    0.01
                };
                (name, average + error_bonus)
            })
            .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(name, _points)| -> &str { name })
            .collect()
    }
}