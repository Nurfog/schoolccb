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

    let repo = Repository::new(pool.clone());
    
    println!("🚀 Starting bootstrap process...");

    // 1. Ensure 'root' role exists and has all permissions
    println!("🔐 Setting up 'root' role and permissions...");
    sqlx::query("INSERT INTO roles (name, description) VALUES ('root', 'Superusuario con acceso total a nivel plataforma') ON CONFLICT (name) DO NOTHING")
        .execute(&pool)
        .await?;

    let root_role_id: i32 = sqlx::query_scalar("SELECT id FROM roles WHERE name = 'root'")
        .fetch_one(&pool)
        .await?;

    // Assign all permissions to root role
    sqlx::query("INSERT INTO role_permissions (role_id, permission_id) SELECT $1, id FROM permissions ON CONFLICT DO NOTHING")
        .bind(root_role_id)
        .execute(&pool)
        .await?;

    // 2. Create School (marked as is_system_admin=true)
    let subdomain = school_name.to_lowercase().replace(" ", "-");
    let school = repo.create_school(school_name, &subdomain, None, true).await?;
    println!("✅ School created: {} (ID: {})", school.name, school.id);

    // 3. Hash Password
    let password_hash = hash_password(admin_password);

    // 4. Create Root User
    let user = repo.create_user(school.id, root_role_id, admin_name, admin_email, &password_hash).await?;
    println!("✅ Root User created: {} (ID: {})", user.name, user.id);

    println!("\n✨ Bootstrap completed successfully! You can now log in at the dashboard.");
    
    Ok(())
}
