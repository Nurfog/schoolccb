use actix_web::{web, App, HttpServer};
use std::env;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use colegio_backend::repository::Repository;
use colegio_backend::handlers;
use actix_cors::Cors;
use actix_web::http::header;

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
        .map_err(|e: sqlx::migrate::MigrateError| std::io::Error::other(e.to_string()))?;

    let repo = Repository::new(pool.clone());

    let port = env::var("PORT").or_else(|_| env::var("RUST_API_PORT")).unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("Starting server on {}", bind_addr);

    let origins = env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost".to_string());
    let allowed_origins: Vec<String> = origins.split(',').map(|s| s.to_string()).collect();

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
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
            .service(handlers::get_saas_stats)
            .service(handlers::list_expiring_licenses)
            .service(handlers::list_countries)
            .service(handlers::list_managed_schools)
            .service(handlers::create_managed_school)
            .service(handlers::get_root_dashboard)
            .service(handlers::list_all_licenses)
            .service(handlers::list_schools_stats)
            .service(handlers::get_school)
            .service(handlers::update_school)
            .service(handlers::bulk_import)
            .service(handlers::update_branding)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
