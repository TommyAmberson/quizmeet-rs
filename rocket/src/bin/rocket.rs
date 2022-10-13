#[macro_use]
extern crate rocket;

#[macro_use(context)]
extern crate quizmeet_rs;

use percent_encoding::percent_decode;
use quizmeet_rs::{entries::Entry, stats::*};
use regex::Regex;
use rocket_dyn_templates::Template;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[get("/parse")]
fn parse() -> String {
    let (team_sums, quizzer_sums) = open_json(Some(String::from("json/*.json")), None).unwrap();

    format!(
        "team_sums: {:#?}\nquizzer_sums: {:#?}",
        team_sums, quizzer_sums
    )
}

fn table_template(regex: Option<Regex>, plain: bool) -> Template {
    let (team_sums, quizzer_sums) = get_lists(Some(String::from("json/*.json")), regex).unwrap();
    let team_avgs: Vec<String> = team_sums
        .iter()
        .map(|v| format!("{:.2}", v.avg()))
        .collect();
    let quizzer_avgs: Vec<String> = quizzer_sums
        .iter()
        .map(|v| format!("{:.2}", v.avg()))
        .collect();

    Template::render(
        if plain { "table" } else { "table-view" },
        context! {
            team_sums,
            team_avgs,
            quizzer_sums,
            quizzer_avgs,
        },
    )
}

#[get("/")]
pub fn table() -> Template {
    table_template(None, false)
}

#[get("/table")]
pub fn table_plain() -> Template {
    table_template(None, true)
}

#[get("/table/div/<div>")]
pub fn table_div(div: &str) -> Template {
    let mut r = String::from("D");
    r += div;
    r += r"Q(?P<q>(\d|\w)+).json$";
    table_template(Some(Regex::new(r.as_str()).unwrap()), true)
}

#[get("/<regex>")]
pub fn table_regex(regex: &str) -> Template {
    let iter = percent_decode(regex.as_bytes());
    let decoded = iter.decode_utf8_lossy().into_owned();
    table_template(Some(Regex::new(&decoded).unwrap()), false)
}

#[get("/table/<regex>")]
pub fn table_regex_plain(regex: &str) -> Template {
    let iter = percent_decode(regex.as_bytes());
    let decoded = iter.decode_utf8_lossy().into_owned();
    table_template(Some(Regex::new(&decoded).unwrap()), true)
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
        .mount(
            "/",
            routes![parse, table, table_plain, table_div, table_regex, table_regex_plain, tera],
        )
        .attach(Template::fairing())
}
