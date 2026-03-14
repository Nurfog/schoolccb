-- ============================================
-- Migración: Seguridad y Auditoría (Fase 7)
-- Fecha: 2026-03-25
-- Descripción: Audit logs mejorado, 2FA, gestión de sesiones
-- Fase: 7.1 - Seguridad
-- ============================================

-- ============================================
-- 1. Audit Logs (Mejorado - agregar columnas faltantes si existen)
-- ============================================
-- Agregar columnas faltantes a audit_logs (creadas por migración 20260313000000)
DO $$ 
BEGIN
    -- Agregar old_values si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'old_values') THEN
        ALTER TABLE audit_logs ADD COLUMN old_values JSONB;
    END IF;
    
    -- Agregar new_values si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'new_values') THEN
        ALTER TABLE audit_logs ADD COLUMN new_values JSONB;
    END IF;
    
    -- Agregar request_method si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'request_method') THEN
        ALTER TABLE audit_logs ADD COLUMN request_method VARCHAR(10);
    END IF;
    
    -- Agregar request_path si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'request_path') THEN
        ALTER TABLE audit_logs ADD COLUMN request_path TEXT;
    END IF;
    
    -- Agregar status_code si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'status_code') THEN
        ALTER TABLE audit_logs ADD COLUMN status_code INTEGER;
    END IF;
    
    -- Agregar duration_ms si no existe
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'duration_ms') THEN
        ALTER TABLE audit_logs ADD COLUMN duration_ms INTEGER;
    END IF;
END $$;

-- Comentario
COMMENT ON TABLE audit_logs IS 'Registro de auditoría para todas las acciones del sistema';
COMMENT ON COLUMN audit_logs.old_values IS 'Valores anteriores (para UPDATE/DELETE)';
COMMENT ON COLUMN audit_logs.new_values IS 'Valores nuevos (para INSERT/UPDATE)';

-- ============================================
-- 2. Gestión de Sesiones de Usuario
-- ============================================
CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE,
    device_info JSONB, -- { "browser": "Chrome", "os": "Windows", "device": "Desktop" }
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    is_current BOOLEAN DEFAULT FALSE, -- Sesión actual del usuario
    last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    revoked_at TIMESTAMPTZ,
    revoke_reason VARCHAR(100)
);

-- Índices para sesiones
CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_refresh_token ON user_sessions(refresh_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_active ON user_sessions(is_active) WHERE is_active = TRUE;
CREATE INDEX IF NOT EXISTS idx_user_sessions_current ON user_sessions(is_current) WHERE is_current = TRUE;
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires ON user_sessions(expires_at);

-- Comentario
COMMENT ON TABLE user_sessions IS 'Gestión de sesiones activas por usuario';
COMMENT ON COLUMN user_sessions.device_info IS 'Información del dispositivo y navegador';
COMMENT ON COLUMN user_sessions.is_current IS 'Indica si es la sesión actual (último login)';

-- ============================================
-- 3. Autenticación 2FA (TOTP)
-- ============================================
CREATE TABLE IF NOT EXISTS user_2fa_secrets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    secret_key VARCHAR(255) NOT NULL, -- Clave secreta encriptada
    backup_codes JSONB, -- Array de códigos de respaldo encriptados
    is_enabled BOOLEAN DEFAULT FALSE,
    enabled_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    failed_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ, -- Bloqueo temporal por múltiples intentos fallidos
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índice para 2FA
CREATE INDEX IF NOT EXISTS idx_user_2fa_secrets_user_id ON user_2fa_secrets(user_id);
CREATE INDEX IF NOT EXISTS idx_user_2fa_secrets_enabled ON user_2fa_secrets(is_enabled) WHERE is_enabled = TRUE;

-- Comentario
COMMENT ON TABLE user_2fa_secrets IS 'Configuración de autenticación 2FA por usuario';
COMMENT ON COLUMN user_2fa_secrets.backup_codes IS 'Códigos de respaldo de un solo uso';

-- ============================================
-- 4. Intentos de Login y Rate Limiting
-- ============================================
CREATE TABLE IF NOT EXISTS login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    success BOOLEAN DEFAULT FALSE,
    user_id UUID REFERENCES users(id), -- NULL si falló
    failure_reason VARCHAR(100), -- 'invalid_password', 'user_not_found', '2fa_failed', 'account_locked'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para login attempts
CREATE INDEX IF NOT EXISTS idx_login_attempts_email ON login_attempts(email);
CREATE INDEX IF NOT EXISTS idx_login_attempts_ip ON login_attempts(ip_address);
CREATE INDEX IF NOT EXISTS idx_login_attempts_created_at ON login_attempts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_login_attempts_failed ON login_attempts(success) WHERE success = FALSE;

-- Comentario
COMMENT ON TABLE login_attempts IS 'Registro de intentos de login para detección de ataques';

-- ============================================
-- 5. Vistas de Auditoría y Seguridad
-- ============================================

-- Vista: Actividad reciente por usuario
CREATE OR REPLACE VIEW user_activity_summary AS
SELECT 
    u.id AS user_id,
    u.name AS user_name,
    u.email,
    COUNT(DISTINCT al.id) AS total_actions,
    COUNT(DISTINCT al.id) FILTER (WHERE al.created_at > NOW() - INTERVAL '24 hours') AS actions_last_24h,
    COUNT(DISTINCT al.id) FILTER (WHERE al.action = 'LOGIN') AS total_logins,
    COUNT(DISTINCT al.id) FILTER (WHERE al.action = 'LOGIN' AND al.created_at > NOW() - INTERVAL '24 hours') AS logins_last_24h,
    MAX(al.created_at) AS last_activity,
    COUNT(DISTINCT al.ip_address) AS unique_ips
FROM users u
LEFT JOIN audit_logs al ON u.id = al.user_id
GROUP BY u.id, u.name, u.email;

-- Vista: Sesiones activas por usuario
CREATE OR REPLACE VIEW active_user_sessions AS
SELECT 
    u.id AS user_id,
    u.name AS user_name,
    u.email,
    COUNT(us.id) AS active_sessions,
    MAX(us.last_activity_at) AS last_activity,
    MAX(us.expires_at) AS earliest_expiry
FROM users u
JOIN user_sessions us ON u.id = us.user_id
WHERE us.is_active = TRUE AND us.expires_at > NOW()
GROUP BY u.id, u.name, u.email;

-- Vista: Intentos de login fallidos por IP (detección de brute force)
CREATE OR REPLACE VIEW suspicious_login_attempts AS
SELECT 
    ip_address,
    email,
    COUNT(*) AS failed_attempts,
    MIN(created_at) AS first_attempt,
    MAX(created_at) AS last_attempt,
    ARRAY_AGG(DISTINCT user_agent) AS user_agents
FROM login_attempts
WHERE success = FALSE
GROUP BY ip_address, email
HAVING COUNT(*) >= 5 -- 5 o más intentos fallidos
ORDER BY failed_attempts DESC;

-- Vista: Usuarios con 2FA habilitado
CREATE OR REPLACE VIEW users_2fa_status AS
SELECT 
    u.id,
    u.name,
    u.email,
    u.role_id,
    COALESCE(t2fa.is_enabled, FALSE) AS has_2fa,
    t2fa.enabled_at,
    t2fa.last_used_at
FROM users u
LEFT JOIN user_2fa_secrets t2fa ON u.id = t2fa.user_id;

-- ============================================
-- 6. Funciones de Auditoría Automática
-- ============================================

-- Función para crear audit log automáticamente
CREATE OR REPLACE FUNCTION create_audit_log(
    p_user_id UUID,
    p_action VARCHAR,
    p_entity VARCHAR,
    p_entity_id UUID,
    p_old_values JSONB DEFAULT NULL,
    p_new_values JSONB DEFAULT NULL,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL,
    p_request_method VARCHAR DEFAULT NULL,
    p_request_path TEXT DEFAULT NULL,
    p_status_code INTEGER DEFAULT NULL,
    p_duration_ms INTEGER DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_audit_id UUID;
BEGIN
    INSERT INTO audit_logs (
        user_id, action, entity, entity_id, old_values, new_values,
        ip_address, user_agent, request_method, request_path,
        status_code, duration_ms
    )
    VALUES (
        p_user_id, p_action, p_entity, p_entity_id, p_old_values, p_new_values,
        p_ip_address, p_user_agent, p_request_method, p_request_path,
        p_status_code, p_duration_ms
    )
    RETURNING id INTO v_audit_id;
    
    RETURN v_audit_id;
END;
$$ LANGUAGE plpgsql;

-- Trigger para auditar cambios en usuarios críticos
CREATE OR REPLACE FUNCTION audit_user_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM create_audit_log(
            NEW.id, 'CREATE', 'users', NEW.id,
            NULL, to_jsonb(NEW), NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        PERFORM create_audit_log(
            NEW.id, 'UPDATE', 'users', NEW.id,
            to_jsonb(OLD), to_jsonb(NEW), NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM create_audit_log(
            OLD.id, 'DELETE', 'users', OLD.id,
            to_jsonb(OLD), NULL, NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger para auditar cambios en licencias SaaS
CREATE OR REPLACE FUNCTION audit_license_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM create_audit_log(
            NEW.id, 'CREATE', 'saas_licenses', NEW.id,
            NULL, to_jsonb(NEW), NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        PERFORM create_audit_log(
            NEW.id, 'UPDATE', 'saas_licenses', NEW.id,
            to_jsonb(OLD), to_jsonb(NEW), NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM create_audit_log(
            OLD.id, 'DELETE', 'saas_licenses', OLD.id,
            to_jsonb(OLD), NULL, NULL, NULL, NULL, NULL, NULL, NULL
        );
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 7. Triggers de Auditoría
-- ============================================

-- Trigger para usuarios (solo cambios críticos)
CREATE TRIGGER trg_audit_users_changes
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_user_changes();

-- Trigger para licencias SaaS
CREATE TRIGGER trg_audit_license_changes
    AFTER INSERT OR UPDATE OR DELETE ON saas_licenses
    FOR EACH ROW EXECUTE FUNCTION audit_license_changes();

-- ============================================
-- 8. Funciones de Gestión de Sesiones
-- ============================================

-- Función para limpiar sesiones expiradas
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM user_sessions
    WHERE expires_at < NOW() OR is_active = FALSE;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Función para verificar si un usuario tiene sesiones activas
CREATE OR REPLACE FUNCTION has_active_sessions(p_user_id UUID)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM user_sessions
        WHERE user_id = p_user_id
          AND is_active = TRUE
          AND expires_at > NOW()
    );
END;
$$ LANGUAGE plpgsql;

-- Función para revocar todas las sesiones de un usuario excepto la actual
CREATE OR REPLACE FUNCTION revoke_all_sessions_except_current(p_user_id UUID, p_current_session_id UUID)
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE user_sessions
    SET is_active = FALSE,
        revoked_at = CURRENT_TIMESTAMP,
        revoke_reason = 'revoked_by_user'
    WHERE user_id = p_user_id
      AND id != p_current_session_id
      AND is_active = TRUE;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 9. Funciones de 2FA
-- ============================================

-- Función para verificar si un usuario tiene 2FA habilitado
CREATE OR REPLACE FUNCTION is_2fa_enabled(p_user_id UUID)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM user_2fa_secrets
        WHERE user_id = p_user_id
          AND is_enabled = TRUE
    );
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 10. Job de Limpieza Automática (requiere pg_cron)
-- ============================================

-- Nota: Para activar la limpieza automática, necesitas la extensión pg_cron
-- Ejecutar en la base de datos:
-- CREATE EXTENSION IF NOT EXISTS pg_cron;

-- Limpieza diaria de sesiones expiradas
-- SELECT cron.schedule('cleanup-sessions', '0 2 * * *', 'SELECT cleanup_expired_sessions();');

-- Limpieza semanal de audit logs antiguos (más de 1 año)
-- SELECT cron.schedule('cleanup-audit-logs', '0 3 * * 0', 'DELETE FROM audit_logs WHERE created_at < NOW() - INTERVAL ''1 year'';');

-- Limpieza mensual de intentos de login antiguos (más de 30 días)
-- SELECT cron.schedule('cleanup-login-attempts', '0 4 1 * *', 'DELETE FROM login_attempts WHERE created_at < NOW() - INTERVAL ''30 days'';');

-- ============================================
-- Fin de la Migración
-- ============================================
