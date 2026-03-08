use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use dotenvy::dotenv;
use colegio_backend::repository::Repository;
use colegio_backend::handlers;
use colegio_backend::HealthResponse;

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

    // Ejecutar migraciones automáticamente
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e: sqlx::migrate::MigrateError| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let repo = Repository::new(pool.clone());

    let port = env::var("PORT").or_else(|_| env::var("RUST_API_PORT")).unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("Starting server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(repo.clone()))
            .service(handlers::index)
            .service(handlers::health)
            .service(handlers::login)
            .service(handlers::register)
            .service(handlers::get_me)
            .service(handlers::list_courses)
            .service(handlers::create_course)
            .service(handlers::list_teachers)
            .service(handlers::create_teacher)
            .service(handlers::list_students)
            .service(handlers::create_student)
            .service(handlers::enroll_student)
            .service(handlers::list_course_students)
            .service(handlers::add_grade)
            .service(handlers::list_course_grades)
            .service(handlers::record_attendance)
            .service(handlers::get_my_report_card)
            .service(handlers::get_active_period)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
