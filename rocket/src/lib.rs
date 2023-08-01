#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

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
