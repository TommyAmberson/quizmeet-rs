use std::sync::Arc;

use polodb_core::Database;
use quizmeet_rs_actix::start_server;
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let db = Database::open_file("test-polo.db").unwrap();
    let db = Arc::new(RwLock::new(db));

    start_server(db.clone(), ("127.0.0.1", 8080)).await
}
