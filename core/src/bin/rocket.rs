#[macro_use]
extern crate rocket;

use quizmeet_rs_rocket::tera;
use rocket::routes;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        log::info!("hi");
    });
    rocket::build()
        .mount("/", routes![tera])
        .attach(Template::fairing())
}
