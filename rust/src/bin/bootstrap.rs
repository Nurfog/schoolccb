use colegio_backend::repository::Repository;
use colegio_backend::auth::hash_password;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 5 {
        println!("Usage: bootstrap <school_name> <admin_name> <admin_email> <admin_password>");
        return Ok(());
    }

    let school_name = &args[1];
    let admin_name = &args[2];
    let admin_email = &args[3];
    let admin_password = &args[4];

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await?;

    let repo = Repository::new(pool);
    
    println!("🚀 Starting bootstrap process...");

    // 1. Create School
    let subdomain = school_name.to_lowercase().replace(" ", "-");
    let school = repo.create_school(school_name, &subdomain, None).await?;
    println!("✅ School created: {} (ID: {})", school.name, school.id);

    // 2. Hash Password
    let password_hash = hash_password(admin_password);

    // 3. Create SuperAdmin User (Role ID 1 is usually admin)
    let user = repo.create_user(school.id, 1, admin_name, admin_email, &password_hash).await?;
    println!("✅ SuperAdmin created: {} (ID: {})", user.name, user.id);

    println!("\n✨ Bootstrap completed successfully! You can now log in at the dashboard.");
    
    Ok(())
}
