#[macro_use] extern crate rocket;

#[macro_use(context)] // Or #[macro_use] to import all macros.
extern crate quizmeet_rs;
use quizmeet_rs::quiz_sum::*;

use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
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

#[get("/")]
pub fn tera() -> Template {
    let name = String::from("Tommy");
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/summary", routes![summary])
        .mount("/tera", routes![tera])
        .attach(Template::fairing())
}

