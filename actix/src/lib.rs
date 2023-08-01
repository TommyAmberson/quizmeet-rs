use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use actix_web::error::ErrorInternalServerError;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use polodb_core::Database;
use tera::Tera;
use tokio::sync::RwLock;

use crate::error::error_handlers;

pub mod error;

pub async fn start_server<A>(db: Arc<RwLock<Database>>, addr: A) -> std::io::Result<()>
where
    A: ToSocketAddrs,
{
    HttpServer::new(move || {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
        log::debug!("{p}");
        let tera = Tera::new(p).unwrap();

        let db = db.clone();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(db))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(addr)?
    .run()
    .await
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, actix_web::Error> {
    let s = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("text", "Welcome!");
        tmpl.render("user.html", &ctx)
            .map_err(|_| ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| ErrorInternalServerError("Template error"))?
    };

    Ok(Html(s))
}
