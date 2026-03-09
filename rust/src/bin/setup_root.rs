use sqlx::{postgres::PgPoolOptions, Row};
use colegio_backend::auth::hash_password;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await?;

    // 1. Clean up the orphan school created by failed bootstrap
    sqlx::query("DELETE FROM schools WHERE name = 'Root Console'")
        .execute(&pool)
        .await?;
    println!("🗑️  Cleaned up orphan Root Console school");

    // 2. Find or create the "root" school (first system admin school)
    let existing_system_school = sqlx::query(
        "SELECT id FROM schools WHERE is_system_admin = true ORDER BY created_at ASC LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;

    let school_id: Uuid = if let Some(row) = existing_system_school {
        let id: Uuid = row.get("id");
        println!("🏢 Using existing system admin school: {}", id);
        id
    } else {
        // create a new root school
        let row = sqlx::query(
            "INSERT INTO schools (name, subdomain, is_system_admin) VALUES ('Root Platform', 'root', true) RETURNING id"
        )
        .fetch_one(&pool)
        .await?;
        let id: Uuid = row.get("id");
        println!("🏢 Created root school: {}", id);
        id
    };

    // 3. Get admin role id
    let role_row = sqlx::query("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
        .fetch_one(&pool)
        .await?;
    let role_id: i32 = role_row.get("id");

    // 4. Check if user exists and update, or create
    let existing_user = sqlx::query("SELECT id FROM users WHERE email = 'juan.allende@gmail.com'")
        .fetch_optional(&pool)
        .await?;

    if let Some(user_row) = existing_user {
        let user_id: Uuid = user_row.get("id");
        let pw_hash = hash_password("apoca11");
        sqlx::query(
            "UPDATE users SET school_id = $1, role_id = $2, password_hash = $3, name = 'Juan Allende' WHERE id = $4"
        )
        .bind(school_id)
        .bind(role_id)
        .bind(&pw_hash)
        .bind(user_id)
        .execute(&pool)
        .await?;
        println!("✅ Updated existing user juan.allende@gmail.com → school_id={}, role=admin", school_id);
    } else {
        let pw_hash = hash_password("apoca11");
        sqlx::query(
            "INSERT INTO users (school_id, role_id, name, email, password_hash) VALUES ($1, $2, 'Juan Allende', 'juan.allende@gmail.com', $3)"
        )
        .bind(school_id)
        .bind(role_id)
        .bind(&pw_hash)
        .execute(&pool)
        .await?;
        println!("✅ Created user juan.allende@gmail.com");
    }

    println!("\n🎉 Root user ready! Login: juan.allende@gmail.com / apoca11");
    Ok(())
}
