#[path = "../iter4/db_access.rs"]
mod db_access;
#[path = "../iter4/errors.rs"]
mod errors;
#[path = "../iter4/handlers.rs"]
mod handlers;
#[path = "../iter4/models.rs"]
mod models;
#[path = "../iter4/routes.rs"]
mod routes;
#[path = "../iter4/state.rs"]
mod state;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::*;
use sqlx::PgPool;
use state::AppState;
use std::sync::Mutex;
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "Rakulang Rocks!".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
