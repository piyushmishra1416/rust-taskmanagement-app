mod models;
mod db;
mod routes;
mod errors;

use actix_web::{web, App, HttpServer};
use crate::db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize in-memory database
    let (task_db, user_db) = init_db();

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(routes::configure)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
