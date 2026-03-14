use actix_web::{dev::Payload, Error as ActixError, FromRequest, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub school_id: String,
    pub is_system_admin: bool,
    pub role: String,
    pub permissions: Vec<String>,
    pub email: String, // Agregado para 2FA
    pub exp: usize,
}

/// Error types for password validation
#[derive(Debug, Clone, PartialEq)]
pub enum PasswordValidationError {
    TooShort,
    NoUppercase,
    NoLowercase,
    NoDigit,
    NoSpecialChar,
}

impl std::fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PasswordValidationError::TooShort => {
                write!(f, "Password must be at least 8 characters")
            }
            PasswordValidationError::NoUppercase => {
                write!(f, "Password must contain at least one uppercase letter")
            }
            PasswordValidationError::NoLowercase => {
                write!(f, "Password must contain at least one lowercase letter")
            }
            PasswordValidationError::NoDigit => {
                write!(f, "Password must contain at least one number")
            }
            PasswordValidationError::NoSpecialChar => {
                write!(f, "Password must contain at least one special character")
            }
        }
    }
}

/// Validate password strength
/// Requirements:
/// - At least 8 characters
/// - At least one uppercase letter
/// - At least one lowercase letter
/// - At least one digit
/// - At least one special character
pub fn validate_password_strength(password: &str) -> Result<(), PasswordValidationError> {
    if password.len() < 8 {
        return Err(PasswordValidationError::TooShort);
    }

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_upper {
        return Err(PasswordValidationError::NoUppercase);
    }
    if !has_lower {
        return Err(PasswordValidationError::NoLowercase);
    }
    if !has_digit {
        return Err(PasswordValidationError::NoDigit);
    }
    if !has_special {
        return Err(PasswordValidationError::NoSpecialChar);
    }

    Ok(())
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn create_jwt(
    user_id: Uuid,
    school_id: Uuid,
    is_system_admin: bool,
    role: &str,
    permissions: Vec<String>,
    email: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        school_id: school_id.to_string(),
        is_system_admin,
        role: role.to_string(),
        permissions,
        email: email.to_string(),
        exp: expiration as usize,
    };

    let secret = env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "secret".to_string());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "secret".to_string());
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

impl FromRequest for Claims {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(token) = auth_header
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
        {
            match decode_jwt(token) {
                Ok(claims) => return ready(Ok(claims)),
                Err(_) => return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized(
            "Missing or invalid authorization header",
        )))
    }
}

// ============================================
// Tests
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "SecurePassword123!";
        let hash = hash_password(password);

        // El hash debe ser diferente cada vez (debido al salt)
        let hash2 = hash_password(password);
        assert_ne!(hash, hash2);

        // La verificación debe funcionar
        assert!(verify_password(password, &hash));
        assert!(verify_password(password, &hash2));
    }

    #[test]
    fn test_password_verification_failure() {
        let password = "SecurePassword123!";
        let wrong_password = "WrongPassword456!";
        let hash = hash_password(password);

        assert!(!verify_password(wrong_password, &hash));
    }

    #[test]
    fn test_validate_password_strength_valid() {
        assert!(validate_password_strength("SecurePass123!").is_ok());
        assert!(validate_password_strength("MyP@ssw0rd").is_ok());
        assert!(validate_password_strength("Test1234!").is_ok());
    }

    #[test]
    fn test_validate_password_strength_too_short() {
        let result = validate_password_strength("Ab1!");
        assert_eq!(result, Err(PasswordValidationError::TooShort));
    }

    #[test]
    fn test_validate_password_strength_no_uppercase() {
        let result = validate_password_strength("password123!");
        assert_eq!(result, Err(PasswordValidationError::NoUppercase));
    }

    #[test]
    fn test_validate_password_strength_no_lowercase() {
        let result = validate_password_strength("PASSWORD123!");
        assert_eq!(result, Err(PasswordValidationError::NoLowercase));
    }

    #[test]
    fn test_validate_password_strength_no_digit() {
        let result = validate_password_strength("Password!");
        assert_eq!(result, Err(PasswordValidationError::NoDigit));
    }

    #[test]
    fn test_validate_password_strength_no_special_char() {
        let result = validate_password_strength("Password123");
        assert_eq!(result, Err(PasswordValidationError::NoSpecialChar));
    }

    #[test]
    fn test_jwt_creation_and_decoding() {
        let user_id = Uuid::new_v4();
        let school_id = Uuid::new_v4();
        let is_system_admin = true;
        let role = "admin";
        let permissions = vec!["users:read".to_string(), "users:write".to_string()];

        // Crear JWT
        let token = create_jwt(
            user_id,
            school_id,
            is_system_admin,
            role,
            permissions.clone(),
            "test@example.com",
        )
        .expect("Failed to create JWT");

        // El token no debe estar vacío
        assert!(!token.is_empty());

        // Decodificar JWT
        let claims = decode_jwt(&token).expect("Failed to decode JWT");

        // Verificar claims
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.school_id, school_id.to_string());
        assert_eq!(claims.is_system_admin, is_system_admin);
        assert_eq!(claims.role, role);
        assert_eq!(claims.permissions, permissions);
    }

    #[test]
    fn test_jwt_invalid_token() {
        let invalid_token = "invalid.token.here";
        let result = decode_jwt(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_expired() {
        // Este test verifica que el token tenga expiración
        let user_id = Uuid::new_v4();
        let school_id = Uuid::new_v4();
        let token = create_jwt(
            user_id,
            school_id,
            false,
            "user",
            vec![],
            "test@example.com",
        )
        .expect("Failed to create JWT");

        let claims = decode_jwt(&token).expect("Failed to decode JWT");

        // El token debe tener una expiración
        assert!(claims.exp > 0);
    }
}
