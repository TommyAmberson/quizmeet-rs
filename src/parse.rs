use crate::quiz_sum::{quiz::Quiz, quizzer::Quizzer, team::Team};
use spreadsheet_ods::{Sheet, Value, WorkBook};

#[derive(Debug)]
pub struct QuizFile<'a> {
    path_str: &'a str,
    // quiz: Quiz,
    sheet: &'a Sheet,
}

impl<'a> QuizFile<'a> {
    pub fn open(&self, path_str: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path_str);
        let wb = spreadsheet_ods::read_ods(path)?;
        let sheet = wb.sheet(1);
        Ok(QuizFile { path_str, sheet })
    }
    pub fn open_default(&self) -> Result<Self, Box<dyn std::error::Error>> {
        self.open("tests/D1Q1.ods")
    }
    pub fn parse(&self) -> Result<Quiz, Box<dyn std::error::Error>> {
        let path = std::path::Path::new("tests/D1Q1.ods");
        let wb = spreadsheet_ods::read_ods(path).unwrap();
        let sheet = wb.sheet(1);
        let quiz = sheet.value(1, 1).as_str_opt().unwrap();
        let mut teams: Vec<Team> = Vec::new();
        for row in 1..4 {
            // let name = String::from(sheet.value(row, 0).as_str_opt().unwrap());
            // let quiz = String::from(quiz);
            // // let quiz = String::from(sheet.value(row, 1).as_str_opt().unwrap());
            // let place = sheet.value(row, 2).as_f64_opt().unwrap();
            // let score = sheet.value(row, 3).as_i32_opt().unwrap();
            // let points = sheet.value(row, 4).as_i32_opt().unwrap();
            // let errors = sheet.value(row, 5).as_i32_opt().unwrap();
            // let team = Team::new(name, quiz, place, score, points, errors);
            teams.push(parse_team_row(sheet, row)?);
        }
        let mut quizzers: Vec<Quizzer> = Vec::new();
        for row in 6..21 {
            // let name = String::from(sheet.value(row, 0).as_str_opt());
            // // if name.
            // let team = String::from(sheet.value(row, 1).as_str_opt().unwrap());
            // let quiz = String::from(quiz);
            // // let quiz = String::from(sheet.value(row, 2).as_str_opt().unwrap());
            // let points = sheet.value(row, 3).as_i32_opt().unwrap();
            // let errors = sheet.value(row, 4).as_i32_opt().unwrap();
            // let jumps = sheet.value(row, 5).as_i32_opt().unwrap();
            // let refer = sheet.value(row, 6).as_i32_opt().unwrap();
            // let ftv = sheet.value(row, 7).as_i32_opt().unwrap();
            // let int = sheet.value(row, 8).as_i32_opt().unwrap();
            // let ma = sheet.value(row, 9).as_i32_opt().unwrap();
            // let q = sheet.value(row, 10).as_i32_opt().unwrap();
            // let sit = sheet.value(row, 11).as_i32_opt().unwrap();
            // let quizzer = Quizzer::new(
            //     name, team, quiz, points, errors, jumps, refer, ftv, int, ma, q, sit,
            // );
            quizzers.push(parse_quizzer_row(sheet, row)?);
        }
        let name = String::from(quiz);
        let quiz = Quiz::new(name, teams, quizzers);

        quiz.print();
        Ok(quiz)
    }

    fn parse_quizzer_row(
        &self,
        sheet: &Sheet,
        row: u32,
    ) -> Result<Quizzer, Box<dyn std::error::Error>> {
        let name = String::from(sheet.value(row, 0).as_str_opt().ok_or("failed")?);
        let team = String::from(sheet.value(row, 1).as_str_opt().ok_or("failed")?);
        let quiz = String::from(sheet.value(row, 2).as_str_opt().ok_or("failed")?);
        let points = sheet.value(row, 3).as_i32_opt().ok_or("failed")?;
        let errors = sheet.value(row, 4).as_i32_opt().ok_or("failed")?;
        let jumps = sheet.value(row, 5).as_i32_opt().ok_or("failed")?;
        let refer = sheet.value(row, 6).as_i32_opt().ok_or("failed")?;
        let ftv = sheet.value(row, 7).as_i32_opt().ok_or("failed")?;
        let int = sheet.value(row, 8).as_i32_opt().ok_or("failed")?;
        let ma = sheet.value(row, 9).as_i32_opt().ok_or("failed")?;
        let q = sheet.value(row, 10).as_i32_opt().ok_or("failed")?;
        let sit = sheet.value(row, 11).as_i32_opt().ok_or("failed")?;
        let quizzer = Quizzer::new(
            name, team, quiz, points, errors, jumps, refer, ftv, int, ma, q, sit,
        );
        Ok(quizzer)
    }
    fn parse_team_row(&self, sheet: &Sheet, row: u32) -> Result<Team, Box<dyn std::error::Error>> {
        let name = String::from(sheet.value(row, 0).as_str_opt().ok_or("failed")?);
        // let quiz = String::from(quiz);
        let quiz = String::from(sheet.value(row, 1).as_str_opt().ok_or("failed")?);
        let place = sheet.value(row, 2).as_f64_opt().ok_or("failed")?;
        let score = sheet.value(row, 3).as_i32_opt().ok_or("failed")?;
        let points = sheet.value(row, 4).as_i32_opt().ok_or("failed")?;
        let errors = sheet.value(row, 5).as_i32_opt().ok_or("failed")?;
        let team = Team::new(name, quiz, place, score, points, errors);
        Ok(team)
    }
}
