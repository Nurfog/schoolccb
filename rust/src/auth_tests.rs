// Tests para el módulo de autenticación
use crate::auth::{
    create_jwt, decode_jwt, hash_password, verify_password, validate_password_strength,
    PasswordValidationError,
};
use uuid::Uuid;

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
    let token = create_jwt(user_id, school_id, is_system_admin, role, permissions.clone())
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
    let token = create_jwt(user_id, school_id, false, "user", vec![])
        .expect("Failed to create JWT");

    let claims = decode_jwt(&token).expect("Failed to decode JWT");
    
    // El token debe tener una expiración
    assert!(claims.exp > 0);
}
