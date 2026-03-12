use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer, web};
use colegio_backend::handlers;
use colegio_backend::repository::Repository;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize tracing subscriber with JSON formatting for production
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Configure connection pool with production-ready settings
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20u32);
    
    let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5u32);
    
    let acquire_timeout = env::var("DATABASE_ACQUIRE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30u64);
    
    let idle_timeout = env::var("DATABASE_IDLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(600u64);
    
    let max_lifetime = env::var("DATABASE_MAX_LIFETIME")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1800u64);

    tracing::info!(
        max_connections = max_connections,
        min_connections = min_connections,
        acquire_timeout = acquire_timeout,
        idle_timeout = idle_timeout,
        max_lifetime = max_lifetime,
        "Configuring database connection pool"
    );

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .idle_timeout(Duration::from_secs(idle_timeout))
        .max_lifetime(Duration::from_secs(max_lifetime))
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    tracing::info!("Database connection pool established");

    // Ejecutar migraciones automáticamente
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e: sqlx::migrate::MigrateError| std::io::Error::other(e.to_string()))?;
    tracing::info!("Database migrations completed successfully");

    let repo = Repository::new(pool.clone());

    let port = env::var("PORT")
        .or_else(|_| env::var("RUST_API_PORT"))
        .unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    tracing::info!(address = %bind_addr, "Starting server");

    let origins = env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost".to_string());
    let allowed_origins: Vec<String> = origins.split(',').map(|s| s.to_string()).collect();

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
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
            .service(handlers::create_country)
            .service(handlers::get_platform_settings)
            .service(handlers::update_platform_setting)
            .service(handlers::list_managed_schools)
            .service(handlers::create_managed_school)
            .service(handlers::get_root_dashboard)
            .service(handlers::list_all_licenses)
            .service(handlers::list_schools_stats)
            .service(handlers::get_school)
            .service(handlers::update_school)
            .service(handlers::bulk_import)
            .service(handlers::update_branding)
            .service(handlers::std_upsert_license)
            .service(handlers::assign_license)
            // Legal Representatives
            .service(handlers::create_legal_representative)
            .service(handlers::list_legal_representatives)
            // Billing & Plans
            .service(handlers::list_plans)
            .service(handlers::get_my_plan)
            .service(handlers::create_checkout)
            .service(handlers::stripe_webhook)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
