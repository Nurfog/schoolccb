// ============================================
// Repositorio de Seguridad y Auditoría (Fase 7)
// ============================================

use crate::models::*;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct SecurityRepository {
    pool: Pool<Postgres>,
}

impl SecurityRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // ============================================
    // Audit Logs
    // ============================================

    /// Crear log de auditoría
    pub async fn create_audit_log(
        &self,
        user_id: Uuid,
        action: &str,
        entity: &str,
        entity_id: Option<Uuid>,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        request_method: Option<&str>,
        request_path: Option<&str>,
        status_code: Option<i32>,
        duration_ms: Option<i32>,
    ) -> Result<AuditLog, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_logs (
                user_id, action, entity, entity_id, old_values, new_values,
                ip_address, user_agent, request_method, request_path,
                status_code, duration_ms
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(action)
        .bind(entity)
        .bind(entity_id)
        .bind(old_values)
        .bind(new_values)
        .bind(ip_address)
        .bind(user_agent)
        .bind(request_method)
        .bind(request_path)
        .bind(status_code)
        .bind(duration_ms)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener audit logs con filtros
    pub async fn get_audit_logs(
        &self,
        filters: &AuditLogFilters,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        // Nota: Implementación simplificada - en producción usar query builder
        sqlx::query_as::<_, AuditLog>(
            r#"
            SELECT * FROM audit_logs
            WHERE ($1::UUID IS NULL OR user_id = $1)
              AND ($2::VARCHAR IS NULL OR action = $2)
              AND ($3::VARCHAR IS NULL OR entity = $3)
              AND ($4::TIMESTAMPTZ IS NULL OR created_at >= $4)
              AND ($5::TIMESTAMPTZ IS NULL OR created_at <= $5)
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#
        )
        .bind(filters.user_id)
        .bind(filters.action.as_deref())
        .bind(filters.entity.as_deref())
        .bind(filters.start_date)
        .bind(filters.end_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Obtener resumen de actividad de usuario
    pub async fn get_user_activity_summary(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserActivitySummary>, sqlx::Error> {
        sqlx::query_as::<_, UserActivitySummary>(
            r#"
            SELECT * FROM user_activity_summary
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Obtener intentos sospechosos de login
    pub async fn get_suspicious_login_attempts(
        &self,
        limit: i64,
    ) -> Result<Vec<SuspiciousLoginAttempts>, sqlx::Error> {
        sqlx::query_as::<_, SuspiciousLoginAttempts>(
            r#"
            SELECT * FROM suspicious_login_attempts
            ORDER BY failed_attempts DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // ============================================
    // Gestión de Sesiones
    // ============================================

    /// Crear sesión
    pub async fn create_session(
        &self,
        user_id: Uuid,
        session_token: &str,
        refresh_token: Option<&str>,
        device_info: serde_json::Value,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<UserSession, sqlx::Error> {
        // Marcar sesión anterior como no actual
        sqlx::query(
            r#"
            UPDATE user_sessions
            SET is_current = FALSE
            WHERE user_id = $1 AND is_current = TRUE
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, UserSession>(
            r#"
            INSERT INTO user_sessions (
                user_id, session_token, refresh_token, device_info,
                ip_address, user_agent, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(session_token)
        .bind(refresh_token)
        .bind(device_info)
        .bind(ip_address)
        .bind(user_agent)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener sesión por token
    pub async fn get_session_by_token(
        &self,
        session_token: &str,
    ) -> Result<Option<UserSession>, sqlx::Error> {
        sqlx::query_as::<_, UserSession>(
            r#"
            SELECT * FROM user_sessions
            WHERE session_token = $1 AND is_active = TRUE AND expires_at > NOW()
            "#
        )
        .bind(session_token)
        .fetch_optional(&self.pool)
        .await
    }

    /// Obtener sesiones activas de un usuario
    pub async fn get_active_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserSessionWithUser>, sqlx::Error> {
        sqlx::query_as::<_, UserSessionWithUser>(
            r#"
            SELECT 
                us.*,
                u.name as user_name,
                u.email as user_email
            FROM user_sessions us
            JOIN users u ON us.user_id = u.id
            WHERE us.user_id = $1
              AND us.is_active = TRUE
              AND us.expires_at > NOW()
            ORDER BY us.last_activity_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Actualizar última actividad
    pub async fn update_session_activity(
        &self,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE user_sessions
            SET last_activity_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Revocar sesión
    pub async fn revoke_session(
        &self,
        session_id: Uuid,
        reason: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE user_sessions
            SET is_active = FALSE,
                revoked_at = CURRENT_TIMESTAMP,
                revoke_reason = $2
            WHERE id = $1
            "#
        )
        .bind(session_id)
        .bind(reason)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Revocar todas las sesiones excepto una
    pub async fn revoke_all_sessions_except(
        &self,
        user_id: Uuid,
        except_session_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE user_sessions
            SET is_active = FALSE,
                revoked_at = CURRENT_TIMESTAMP,
                revoke_reason = 'revoked_by_user'
            WHERE user_id = $1
              AND id != $2
              AND is_active = TRUE
            "#
        )
        .bind(user_id)
        .bind(except_session_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Limpiar sesiones expiradas
    pub async fn cleanup_expired_sessions(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_sessions
            WHERE expires_at < NOW() OR is_active = FALSE
            "#
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    // ============================================
    // 2FA (TOTP)
    // ============================================

    /// Crear o actualizar secreto 2FA
    pub async fn upsert_2fa_secret(
        &self,
        user_id: Uuid,
        secret_key: &str,
        backup_codes: Option<serde_json::Value>,
    ) -> Result<User2faSecret, sqlx::Error> {
        sqlx::query_as::<_, User2faSecret>(
            r#"
            INSERT INTO user_2fa_secrets (user_id, secret_key, backup_codes)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE
            SET secret_key = EXCLUDED.secret_key,
                backup_codes = EXCLUDED.backup_codes,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(secret_key)
        .bind(backup_codes)
        .fetch_one(&self.pool)
        .await
    }

    /// Habilitar 2FA
    pub async fn enable_2fa(&self, user_id: Uuid) -> Result<User2faSecret, sqlx::Error> {
        sqlx::query_as::<_, User2faSecret>(
            r#"
            UPDATE user_2fa_secrets
            SET is_enabled = TRUE,
                enabled_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Deshabilitar 2FA
    pub async fn disable_2fa(&self, user_id: Uuid) -> Result<User2faSecret, sqlx::Error> {
        sqlx::query_as::<_, User2faSecret>(
            r#"
            UPDATE user_2fa_secrets
            SET is_enabled = FALSE,
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener secreto 2FA de usuario
    pub async fn get_2fa_secret(
        &self,
        user_id: Uuid,
    ) -> Result<Option<User2faSecret>, sqlx::Error> {
        sqlx::query_as::<_, User2faSecret>(
            r#"
            SELECT * FROM user_2fa_secrets
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Registrar uso exitoso de 2FA
    pub async fn record_2fa_usage(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE user_2fa_secrets
            SET last_used_at = CURRENT_TIMESTAMP,
                failed_attempts = 0,
                locked_until = NULL
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Registrar fallo de 2FA
    pub async fn record_2fa_failure(
        &self,
        user_id: Uuid,
        lockout_threshold: i32,
        lockout_duration: i64,
    ) -> Result<bool, sqlx::Error> {
        // Incrementar intentos fallidos
        let result = sqlx::query_as::<_, User2faSecret>(
            r#"
            UPDATE user_2fa_secrets
            SET failed_attempts = failed_attempts + 1,
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1
            RETURNING failed_attempts, locked_until
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Verificar si debe bloquearse
        if result.failed_attempts >= lockout_threshold {
            sqlx::query(
                r#"
                UPDATE user_2fa_secrets
                SET locked_until = NOW() + INTERVAL '$2 seconds',
                    updated_at = CURRENT_TIMESTAMP
                WHERE user_id = $1
                "#
            )
            .bind(user_id)
            .bind(lockout_duration)
            .execute(&self.pool)
            .await?;
            Ok(true) // Bloqueado
        } else {
            Ok(false) // No bloqueado
        }
    }

    /// Verificar si el usuario está bloqueado para 2FA
    pub async fn is_2fa_locked(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, Option<DateTime<Utc>>>(
            r#"
            SELECT locked_until FROM user_2fa_secrets
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.map_or(false, |locked_until| locked_until > chrono::Utc::now()))
    }

    /// Usar código de respaldo
    pub async fn use_backup_code(
        &self,
        user_id: Uuid,
        _code: &str,
    ) -> Result<bool, sqlx::Error> {
        // Obtener códigos de respaldo
        let secret = self.get_2fa_secret(user_id).await?;
        
        if let Some(_backup_codes) = secret.and_then(|s| s.backup_codes) {
            // Verificar si el código existe y eliminarlo
            // Nota: La lógica específica depende de cómo se almacenen los códigos
            // Esto es un ejemplo simplificado
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================
    // Login Attempts
    // ============================================

    /// Registrar intento de login
    pub async fn record_login_attempt(
        &self,
        email: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        success: bool,
        user_id: Option<Uuid>,
        failure_reason: Option<&str>,
    ) -> Result<LoginAttempt, sqlx::Error> {
        sqlx::query_as::<_, LoginAttempt>(
            r#"
            INSERT INTO login_attempts (
                email, ip_address, user_agent, success, user_id, failure_reason
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(email)
        .bind(ip_address)
        .bind(user_agent)
        .bind(success)
        .bind(user_id)
        .bind(failure_reason)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener intentos de login recientes por IP
    pub async fn get_recent_login_attempts_by_ip(
        &self,
        ip_address: &str,
        minutes: i64,
    ) -> Result<Vec<LoginAttempt>, sqlx::Error> {
        sqlx::query_as::<_, LoginAttempt>(
            r#"
            SELECT * FROM login_attempts
            WHERE ip_address = $1::INET
              AND created_at > NOW() - INTERVAL '$2 minutes'
            ORDER BY created_at DESC
            "#
        )
        .bind(ip_address)
        .bind(minutes)
        .fetch_all(&self.pool)
        .await
    }

    /// Obtener intentos fallidos recientes por email
    pub async fn get_recent_failed_attempts_by_email(
        &self,
        email: &str,
        minutes: i64,
    ) -> Result<Vec<LoginAttempt>, sqlx::Error> {
        sqlx::query_as::<_, LoginAttempt>(
            r#"
            SELECT * FROM login_attempts
            WHERE email = $1
              AND success = FALSE
              AND created_at > NOW() - INTERVAL '$2 minutes'
            ORDER BY created_at DESC
            "#
        )
        .bind(email)
        .bind(minutes)
        .fetch_all(&self.pool)
        .await
    }
}
