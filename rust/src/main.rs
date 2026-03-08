use actix_web::{web, App, HttpResponse, HttpServer, get};
use serde::{Deserialize, Serialize};
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    status: String,
    message: String,
    db_connected: Option<bool>,
}

#[get("/health")]
async fn health(db_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let db_status = sqlx::query("SELECT 1")
        .execute(db_pool.get_ref())
        .await;

    let (message, db_connected) = match db_status {
        Ok(_) => ("Server is running and DB is connected".to_string(), true),
        Err(e) => (format!("Server is running but DB error: {}", e), false),
    };

    HttpResponse::Ok().json(HealthResponse {
        status: if db_connected { "ok".to_string() } else { "warning".to_string() },
        message,
        db_connected: Some(db_connected),
    })
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        message: "Colegio Backend API v1.0".to_string(),
        db_connected: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let port = env::var("PORT").or_else(|_| env::var("RUST_API_PORT")).unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("Starting server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(health)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
