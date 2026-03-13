use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer, web};
use colegio_backend::config::AppConfig;
use colegio_backend::handlers;
use colegio_backend::repository::Repository;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load and validate configuration from environment variables
    let config = AppConfig::from_env().unwrap_or_else(|e| {
        eprintln!("❌ Configuration error: {}", e);
        std::process::exit(1);
    });

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("❌ Configuration validation failed: {}", e);
        std::process::exit(1);
    }

    // Initialize tracing subscriber with JSON formatting for production
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.rust_log.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!(
        max_connections = config.database_max_connections,
        min_connections = config.database_min_connections,
        acquire_timeout = config.database_acquire_timeout.as_secs(),
        idle_timeout = config.database_idle_timeout.as_secs(),
        max_lifetime = config.database_max_lifetime.as_secs(),
        "Configuring database connection pool"
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .acquire_timeout(config.database_acquire_timeout)
        .idle_timeout(config.database_idle_timeout)
        .max_lifetime(config.database_max_lifetime)
        .connect(&config.database_url)
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

    let bind_addr = format!("{}:{}", config.host, config.port);

    tracing::info!(address = %bind_addr, "Starting server");

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

        for origin in &config.cors_origins {
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
