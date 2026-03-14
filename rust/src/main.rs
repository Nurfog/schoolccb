use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer, web};
use colegio_backend::ai_module::{AIClient, AIConfig};
use colegio_backend::communications_repository::CommunicationsRepository;
use colegio_backend::config::AppConfig;
use colegio_backend::handlers;
use colegio_backend::repository::Repository;
use colegio_backend::security_repository::SecurityRepository;
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

    // Crear cliente de IA
    let ai_config = AIConfig::from_env();
    let ai_client = web::Data::new(AIClient::new(ai_config));

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
            .app_data(web::Data::new(CommunicationsRepository::new(pool.clone())))
            .app_data(web::Data::new(SecurityRepository::new(pool.clone())))
            .app_data(ai_client.clone())
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
            // Communications (Fase 6)
            .service(handlers::get_notifications)
            .service(handlers::get_unread_count)
            .service(handlers::mark_notification_read)
            .service(handlers::mark_all_notifications_read)
            .service(handlers::delete_notification)
            .service(handlers::get_notification_preferences)
            .service(handlers::update_notification_preferences)
            .service(handlers::list_templates)
            .service(handlers::get_template)
            .service(handlers::list_announcements)
            .service(handlers::get_announcement)
            .service(handlers::confirm_announcement_reading)
            .service(handlers::get_announcement_stats)
            .service(handlers::create_announcement)
            .service(handlers::publish_announcement)
            .service(handlers::update_announcement)
            .service(handlers::delete_announcement)
            .service(handlers::create_attendance_justification)
            .service(handlers::get_student_justifications)
            .service(handlers::get_pending_justifications)
            .service(handlers::review_justification)
            .service(handlers::count_pending_justifications)
            // Security & Audit (Fase 7)
            .service(handlers::get_audit_logs)
            .service(handlers::get_user_activity)
            .service(handlers::get_suspicious_logins)
            .service(handlers::get_my_sessions)
            .service(handlers::revoke_session)
            .service(handlers::revoke_all_other_sessions)
            .service(handlers::setup_2fa)
            .service(handlers::verify_2fa)
            .service(handlers::disable_2fa)
            .service(handlers::get_2fa_status)
            .service(handlers::login_2fa_verify)
            // PDF Generation
            .service(handlers::generate_report_card_pdf)
            .service(handlers::generate_certificate_pdf)
            // IA/ML (Fase 9)
            .service(handlers::ai_chatbot)
            .service(handlers::ai_analyze_dropout_risk)
            .service(handlers::ai_generate_feedback)
            .service(handlers::ai_classify_query)
            .service(handlers::ai_summarize)
            .service(handlers::ai_analyze_sentiment)
            .service(handlers::ai_transcribe)
            .service(handlers::ai_status)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
