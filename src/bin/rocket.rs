#[macro_use]
extern crate rocket;

#[macro_use(context)]
extern crate quizmeet_rs;

use quizmeet_rs::{entries::*, io::*, quiz_sum::*};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/summary")]
fn summary() -> String {
    // println!("{}", quiz_sum::hello());
    let mut sum = Summary::new();
    sum.open_ods().unwrap();
    // dbg!(&sum);
    // dbg!(sum.get_team_prelims(1));
    let mut result = String::from("");
    let t = sum.get_team_order(|q| q.div == 1 && matches!(q.quiz, QuizType::Preliminary(_)));
    dbg!(&t);
    result.push_str(&format!("{:?}", &t));
    let q = sum.get_quizzer_order(|q| q.div == 1);
    dbg!(&q);
    result.push_str(&format!("{:?}", q));

    result
}

#[get("/parse")]
fn parse() -> String {
    let g = String::from("json/*.json");
    let mut team_entries: Vec<TeamEntry> = Vec::new();
    let mut quizzer_entries: Vec<QuizzerEntry> = Vec::new();
    from_glob(&g, |entry| {
        let result = read(entry.as_path())?;
        team_entries.extend(result.0);
        quizzer_entries.extend(result.1);
        Ok(())
    })
    .unwrap();
    let team_sums = group_and_sum(team_entries).unwrap();
    let quizzer_sums = group_and_sum(quizzer_entries).unwrap();

    format!(
        "team_sums: {:#?}\nquizzer_sums: {:#?}",
        team_sums, quizzer_sums
    )
}

#[get("/tera")]
pub fn tera() -> Template {
    let name = String::from("Tommy");
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, summary, parse, tera])
        .attach(Template::fairing())
}
