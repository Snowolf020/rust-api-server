use std::env;
use actix_web::{web, App, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

mod db;
mod models;
mod routes;
mod utils;

#[derive(Serialize, Deserialize)]
struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let app_state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(routes::api_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}