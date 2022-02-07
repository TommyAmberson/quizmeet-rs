use crate::quiz_sum::{quiz::Quiz, quizzer::Quizzer, team::Team};
use spreadsheet_ods::{error::OdsError, Sheet, Value, WorkBook};

#[derive(Debug)]
pub struct QuizFile {
    // path_str: &'a str,
    // quiz: Quiz,
    wb: WorkBook,
    // sheet: &'a Sheet,
}

impl QuizFile {
    pub fn open(path_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path_str);
        let wb = spreadsheet_ods::read_ods(path)?;
        if wb.num_sheets() < 2 {
            return Err(Box::new(OdsError::Ods(String::from("err"))));
            // return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "err")));
        }
        // let sheet = wb.sheet(1);
        // Ok(QuizFile { path_str, wb })
        Ok(QuizFile { wb })
    }
    // pub fn open_test() -> Result<Self, Box<dyn std::error::Error>> {
    //     open("tests/D1Q1.ods")
    // }
    pub fn parse(&self) -> Result<Quiz, Box<dyn std::error::Error>> {
        let quiz = self.wb.sheet(1).value(1, 1).as_str_opt().ok_or("failed")?;
        let mut teams: Vec<Team> = Vec::new();
        for row in 1..4 {
            let team = match self.parse_team_row(row) {
                Ok(team) => team,
                Err(_) => continue,
            };
            teams.push(team);
        }
        let mut quizzers: Vec<Quizzer> = Vec::new();
        for row in 6..21 {
            let quizzer = match self.parse_quizzer_row(row) {
                Ok(quizzer) => quizzer,
                Err(_) => continue,
            };
            quizzers.push(quizzer);
        }
        let name = String::from(quiz);
        let quiz = Quiz::new(name, teams, quizzers);
        // quiz.print();
        Ok(quiz)
    }

    fn parse_quizzer_row(&self, row: u32) -> Result<Quizzer, Box<dyn std::error::Error>> {
        let sheet = self.wb.sheet(1);
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
    fn parse_team_row(&self, row: u32) -> Result<Team, Box<dyn std::error::Error>> {
        let sheet = self.wb.sheet(1);
        let name = String::from(sheet.value(row, 0).as_str_opt().ok_or("failed")?);
        let quiz = String::from(sheet.value(row, 1).as_str_opt().ok_or("failed")?);
        let place = sheet.value(row, 2).as_f64_opt().ok_or("failed")?;
        let score = sheet.value(row, 3).as_i32_opt().ok_or("failed")?;
        let points = sheet.value(row, 4).as_i32_opt().ok_or("failed")?;
        let errors = sheet.value(row, 5).as_i32_opt().ok_or("failed")?;
        let team = Team::new(name, quiz, place, score, points, errors);
        Ok(team)
    }
}
