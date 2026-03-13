use std::env;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("Invalid value for {0}: {1}")]
    Invalid(&'static str, String),
    #[error("Parse error: {0}")]
    Parse(#[from] std::env::VarError),
    #[error("Invalid port: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
    pub cors_origins: Vec<String>,
    pub rust_log: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
    pub database_acquire_timeout: Duration,
    pub database_idle_timeout: Duration,
    pub database_max_lifetime: Duration,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Validate DATABASE_URL
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::Missing("DATABASE_URL"))?;
        
        if !database_url.starts_with("postgresql://") && !database_url.starts_with("postgres://") {
            return Err(ConfigError::Invalid("DATABASE_URL", "Must start with postgresql:// or postgres://".to_string()));
        }

        // Validate JWT_SECRET_KEY
        let jwt_secret = env::var("JWT_SECRET_KEY")
            .map_err(|_| ConfigError::Missing("JWT_SECRET_KEY"))?;
        
        if jwt_secret.len() < 32 {
            return Err(ConfigError::Invalid(
                "JWT_SECRET_KEY",
                "Must be at least 32 characters for security".to_string()
            ));
        }

        // Parse port
        let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let port: u16 = port_str.parse()?;
        
        if port == 0 {
            return Err(ConfigError::Invalid("PORT", "Cannot be 0".to_string()));
        }

        // Host
        let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        // CORS origins
        let cors_origins_str = env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost".to_string());
        let cors_origins: Vec<String> = cors_origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Validate CORS in production
        let node_env = env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string());
        if node_env == "production" {
            for origin in &cors_origins {
                if origin == "*" {
                    eprintln!("⚠️  WARNING: CORS is set to '*' in production! This is a security risk.");
                }
            }
        }

        // RUST_LOG
        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx=warn,actix_web=info".to_string());

        // Database pool configuration
        let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);

        let database_min_connections: u32 = env::var("DATABASE_MIN_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let acquire_timeout_secs: u64 = env::var("DATABASE_ACQUIRE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        let idle_timeout_secs: u64 = env::var("DATABASE_IDLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(600);

        let max_lifetime_secs: u64 = env::var("DATABASE_MAX_LIFETIME")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1800);

        Ok(Self {
            database_url,
            jwt_secret,
            port,
            host,
            cors_origins,
            rust_log,
            database_max_connections,
            database_min_connections,
            database_acquire_timeout: Duration::from_secs(acquire_timeout_secs),
            database_idle_timeout: Duration::from_secs(idle_timeout_secs),
            database_max_lifetime: Duration::from_secs(max_lifetime_secs),
        })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Additional validation can be added here
        if self.database_max_connections < self.database_min_connections {
            return Err(ConfigError::Invalid(
                "DATABASE_MAX_CONNECTIONS",
                "Must be greater than or equal to DATABASE_MIN_CONNECTIONS".to_string()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_database_url() {
        env::remove_var("DATABASE_URL");
        env::set_var("JWT_SECRET_KEY", "a_very_secure_secret_that_is_long_enough");
        
        let result = AppConfig::from_env();
        assert!(matches!(result, Err(ConfigError::Missing("DATABASE_URL"))));
    }

    #[test]
    fn test_short_jwt_secret() {
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("JWT_SECRET_KEY", "short");
        
        let result = AppConfig::from_env();
        assert!(matches!(result, Err(ConfigError::Invalid("JWT_SECRET_KEY", _))));
    }

    #[test]
    fn test_valid_config() {
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("JWT_SECRET_KEY", "a_very_secure_secret_that_is_long_enough_for_security");
        env::set_var("PORT", "8080");
        
        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.port, 8080);
        assert_eq!(config.database_max_connections, 20);
        assert_eq!(config.database_min_connections, 5);
    }
}
