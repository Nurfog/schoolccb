-- Fase 11: Capa de Empresa SaaS y Administración Avanzada

-- 1. Tabla de Países
CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    code CHAR(2) NOT NULL UNIQUE -- ISO 3166-1 alpha-2
);

-- Sembrar algunos países iniciales
INSERT INTO countries (name, code) VALUES 
('Colombia', 'CO'),
('México', 'MX'),
('España', 'ES'),
('Argentina', 'AR'),
('Estados Unidos', 'US')
ON CONFLICT DO NOTHING;

-- 2. Actualizar Tabla de Colegios
ALTER TABLE schools ADD COLUMN IF NOT EXISTS country_id INTEGER REFERENCES countries(id);
ALTER TABLE schools ADD COLUMN IF NOT EXISTS is_system_admin BOOLEAN DEFAULT false; -- Indica si es el colegio "dueño" del SaaS

-- 3. Tabla de Licencias SaaS
CREATE TABLE IF NOT EXISTS saas_licenses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    plan_type VARCHAR(20) NOT NULL DEFAULT 'basic', -- basic, premium, enterprise
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, expired, trial, suspended
    start_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expiry_date TIMESTAMP WITH TIME ZONE NOT NULL,
    auto_renew BOOLEAN DEFAULT false,
    card_last4 CHAR(4),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 4. Permisos para Administración SaaS (Global)
INSERT INTO permissions (name, description) VALUES
('saas:manage_schools', 'Crear y gestionar colegios en el sistema'),
('saas:manage_licenses', 'Gestionar planes y cobros de licencias'),
('saas:view_dashboard', 'Ver métricas globales del SaaS'),
('admin:manage_roles', 'Crear y editar roles personalizados en el colegio'),
('admin:bulk_import', 'Importar usuarios masivamente vía CSV/Excel')
ON CONFLICT (name) DO NOTHING;

-- Al asignar privilegios, aseguramos que el SuperAdmin tenga todo
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'admin' AND p.name LIKE 'saas:%'
ON CONFLICT DO NOTHING;

-- Los administradores de colegio creados por el SaaS reciben permisos locales
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'admin' AND p.name IN ('admin:manage_roles', 'admin:bulk_import')
ON CONFLICT DO NOTHING;
