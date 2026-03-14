-- ============================================
-- Migración: Datos Iniciales (Seed)
-- Fecha: 2026-03-31
-- Descripción: Crea datos básicos: roles, colegio principal, permisos
-- ============================================

-- ============================================
-- 1. Países adicionales (los básicos ya existen)
-- ============================================
INSERT INTO countries (name, code)
VALUES 
    ('Chile', 'CL'),
    ('Perú', 'PE'),
    ('Ecuador', 'EC'),
    ('Panamá', 'PA'),
    ('Costa Rica', 'CR')
ON CONFLICT (code) DO NOTHING;

-- ============================================
-- 2. Roles del Sistema
-- ============================================
INSERT INTO roles (name, description, is_system)
VALUES 
    ('root', 'Administrador de la plataforma SaaS', TRUE),
    ('admin', 'Administrador del colegio', TRUE),
    ('teacher', 'Profesor', TRUE),
    ('student', 'Alumno', TRUE),
    ('parent', 'Padre de familia', TRUE),
    ('staff', 'Personal administrativo', TRUE)
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- 3. Permisos Base
-- ============================================
INSERT INTO permissions (name, description, resource, action)
VALUES 
    ('users.view', 'Ver usuarios', 'users', 'view'),
    ('users.create', 'Crear usuarios', 'users', 'create'),
    ('users.edit', 'Editar usuarios', 'users', 'edit'),
    ('users.delete', 'Eliminar usuarios', 'users', 'delete'),
    ('students.view', 'Ver estudiantes', 'students', 'view'),
    ('students.create', 'Crear estudiantes', 'students', 'create'),
    ('students.edit', 'Editar estudiantes', 'students', 'edit'),
    ('teachers.view', 'Ver profesores', 'teachers', 'view'),
    ('teachers.create', 'Crear profesores', 'teachers', 'create'),
    ('courses.view', 'Ver cursos', 'courses', 'view'),
    ('courses.create', 'Crear cursos', 'courses', 'create'),
    ('grades.view', 'Ver calificaciones', 'grades', 'view'),
    ('grades.edit', 'Editar calificaciones', 'grades', 'edit'),
    ('attendance.view', 'Ver asistencia', 'attendance', 'view'),
    ('attendance.edit', 'Editar asistencia', 'attendance', 'edit'),
    ('finance.view', 'Ver finanzas', 'finance', 'view'),
    ('finance.manage', 'Gestionar finanzas', 'finance', 'manage'),
    ('reports.view', 'Ver reportes', 'reports', 'view'),
    ('settings.view', 'Ver configuración', 'settings', 'view'),
    ('settings.edit', 'Editar configuración', 'settings', 'edit')
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- 4. Colegio Principal (School)
-- ============================================
INSERT INTO schools (
    name,
    razon_social,
    subdomain,
    email_contacto,
    telefono,
    country_id,
    is_system_admin
)
SELECT
    'Colegio Principal',
    'Colegio Principal S.A.S.',
    'principal',
    'info@colegioprincipal.edu.co',
    '+57 604 123 4567',
    (SELECT id FROM countries WHERE code = 'CO' LIMIT 1),
    TRUE
WHERE NOT EXISTS (SELECT 1 FROM schools WHERE subdomain = 'principal');

-- ============================================
-- 5. Licencia SaaS para el colegio
-- ============================================
INSERT INTO saas_licenses (
    school_id,
    plan_type,
    status,
    start_date,
    expiry_date,
    auto_renew
)
SELECT
    s.id,
    'enterprise',
    'active',
    CURRENT_DATE,
    CURRENT_DATE + INTERVAL '1 year',
    TRUE
FROM schools s
WHERE s.subdomain = 'principal'
AND NOT EXISTS (SELECT 1 FROM saas_licenses WHERE school_id = s.id);

-- ============================================
-- 6. Asignar permisos a rol admin
-- ============================================
INSERT INTO role_permissions (role_id, permission_id)
SELECT 
    r.id,
    p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'admin'
AND p.resource IN ('users', 'students', 'teachers', 'courses', 'grades', 'attendance', 'reports')
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Asignar todos los permisos a root
INSERT INTO role_permissions (role_id, permission_id)
SELECT 
    r.id,
    p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'root'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- ============================================
-- 7. Platform Settings
-- ============================================
INSERT INTO platform_settings (setting_key, setting_value, setting_type, description)
VALUES
    ('platform.name', 'SchoolCCB SaaS', 'string', 'Nombre de la plataforma'),
    ('platform.version', '1.0.0', 'string', 'Versión de la plataforma'),
    ('platform.timezone', 'America/Bogota', 'string', 'Zona horaria por defecto'),
    ('platform.locale', 'es_CO', 'string', 'Idioma por defecto'),
    ('platform.currency', 'COP', 'string', 'Moneda por defecto'),
    ('email.from_name', 'SchoolCCB', 'string', 'Nombre para emails salientes'),
    ('email.from_address', 'noreply@schoolccb.com', 'string', 'Email para notificaciones'),
    ('security.password_min_length', '8', 'string', 'Longitud mínima de contraseña'),
    ('security.session_timeout', '3600', 'string', 'Timeout de sesión en segundos'),
    ('security.max_login_attempts', '5', 'string', 'Intentos máximos de login')
ON CONFLICT (setting_key) DO NOTHING;

-- ============================================
-- Fin de la Migración
-- ============================================
