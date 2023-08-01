use actix_web::{middleware, web, App, HttpServer};
use quizmeet_rs_actix::error::error_handlers;
use quizmeet_rs_actix::index;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/**/*");
        log::info!("{p}");
        let tera = Tera::new(p).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
