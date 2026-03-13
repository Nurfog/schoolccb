-- ============================================
-- Migración: Índices de Rendimiento y Audit Logs
-- Fecha: 2026-03-13
-- Descripción: Agrega índices para optimizar consultas críticas
--              y crea tabla de audit_logs para seguridad
-- ============================================

-- ============================================
-- Índices para Usuarios (búsquedas por email y escuela)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_school_id ON users(school_id);
CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

-- ============================================
-- Índices para Licencias SaaS (consultas de expiración y estado)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_saas_licenses_school_id ON saas_licenses(school_id);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_expiry_date ON saas_licenses(expiry_date);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_status ON saas_licenses(status);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_plan_type ON saas_licenses(plan_type);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_auto_renew ON saas_licenses(auto_renew);

-- ============================================
-- Índices para Cursos (filtrado por escuela y profesor)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_courses_school_id ON courses(school_id);
CREATE INDEX IF NOT EXISTS idx_courses_teacher_id ON courses(teacher_id);
CREATE INDEX IF NOT EXISTS idx_courses_created_at ON courses(created_at);

-- ============================================
-- Índices para Calificaciones (consultas por estudiante y curso)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_grades_student_id ON grades(student_id);
CREATE INDEX IF NOT EXISTS idx_grades_course_id ON grades(course_id);
CREATE INDEX IF NOT EXISTS idx_grades_period_id ON grades(period_id);
CREATE INDEX IF NOT EXISTS idx_grades_created_at ON grades(created_at);

-- ============================================
-- Índices para Asistencia (consultas por fecha y estudiante)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_attendance_student_id ON attendance(student_id);
CREATE INDEX IF NOT EXISTS idx_attendance_course_id ON attendance(course_id);
CREATE INDEX IF NOT EXISTS idx_attendance_date ON attendance(date);
CREATE INDEX IF NOT EXISTS idx_attendance_status ON attendance(status);

-- ============================================
-- Índices para Matrículas (Enrollments)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_enrollments_student_id ON enrollments(student_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_course_id ON enrollments(course_id);

-- ============================================
-- Índices para Profesores y Estudiantes
-- ============================================
CREATE INDEX IF NOT EXISTS idx_teachers_user_id ON teachers(user_id);
CREATE INDEX IF NOT EXISTS idx_students_user_id ON students(user_id);
CREATE INDEX IF NOT EXISTS idx_students_parent_id ON students(parent_id);

-- ============================================
-- Índices para Colegios (Schools)
-- ============================================
CREATE INDEX IF NOT EXISTS idx_schools_subdomain ON schools(subdomain);
CREATE INDEX IF NOT EXISTS idx_schools_country_id ON schools(country_id);
CREATE INDEX IF NOT EXISTS idx_schools_is_system_admin ON schools(is_system_admin);

-- ============================================
-- Índices para Permisos y Roles
-- ============================================
CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_permission_id ON role_permissions(permission_id);

-- ============================================
-- Índices para Platform Settings
-- ============================================
CREATE INDEX IF NOT EXISTS idx_platform_settings_key ON platform_settings(setting_key);

-- ============================================
-- Tabla de Audit Logs (Seguridad y Cumplimiento)
-- ============================================
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    action VARCHAR(100) NOT NULL,
    entity VARCHAR(100) NOT NULL,
    entity_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para audit_logs
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity ON audit_logs(entity);
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity_id ON audit_logs(entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);

-- Comentario de la tabla
COMMENT ON TABLE audit_logs IS 'Registro de auditoría para acciones críticas del sistema';
COMMENT ON COLUMN audit_logs.user_id IS 'ID del usuario que realizó la acción';
COMMENT ON COLUMN audit_logs.action IS 'Tipo de acción realizada (CREATE, UPDATE, DELETE, LOGIN, etc.)';
COMMENT ON COLUMN audit_logs.entity IS 'Entidad afectada (users, courses, grades, etc.)';
COMMENT ON COLUMN audit_logs.entity_id IS 'ID de la entidad afectada';
COMMENT ON COLUMN audit_logs.details IS 'Detalles adicionales en formato JSON';
COMMENT ON COLUMN audit_logs.ip_address IS 'Dirección IP desde la que se realizó la acción';
COMMENT ON COLUMN audit_logs.user_agent IS 'User agent del cliente';

-- ============================================
-- Función para logging automático de auditoría
-- ============================================
CREATE OR REPLACE FUNCTION log_audit_action(
    p_user_id UUID,
    p_action VARCHAR,
    p_entity VARCHAR,
    p_entity_id UUID,
    p_details JSONB,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_audit_id UUID;
BEGIN
    INSERT INTO audit_logs (user_id, action, entity, entity_id, details, ip_address, user_agent)
    VALUES (p_user_id, p_action, p_entity, p_entity_id, p_details, p_ip_address, p_user_agent)
    RETURNING id INTO v_audit_id;
    
    RETURN v_audit_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- Vista para dashboard de auditoría
-- ============================================
CREATE OR REPLACE VIEW audit_logs_summary AS
SELECT 
    DATE(created_at) as log_date,
    action,
    entity,
    COUNT(*) as action_count,
    COUNT(DISTINCT user_id) as unique_users
FROM audit_logs
GROUP BY DATE(created_at), action, entity
ORDER BY log_date DESC, action_count DESC;

-- ============================================
-- Trigger para limpiar logs antiguos (opcional, se puede activar después)
-- ============================================
-- Nota: Para activar la limpieza automática, ejecutar:
-- SELECT cron.schedule('cleanup-audit-logs', '0 2 * * 0', 'DELETE FROM audit_logs WHERE created_at < NOW() - INTERVAL ''90 days''');
