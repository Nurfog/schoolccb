-- ============================================
-- Migración: Configuración de Plataforma
-- ============================================

-- Tabla para configuración global de la plataforma
CREATE TABLE IF NOT EXISTS platform_settings (
    id SERIAL PRIMARY KEY,
    setting_key VARCHAR(100) UNIQUE NOT NULL,
    setting_value TEXT NOT NULL,
    setting_type VARCHAR(50) DEFAULT 'string',
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar configuración por defecto
INSERT INTO platform_settings (setting_key, setting_value, setting_type, description) VALUES
    ('platform_name', 'SchoolCCB', 'string', 'Nombre de la plataforma'),
    ('platform_logo', '', 'string', 'Logo de la plataforma en base64'),
    ('platform_favicon', '', 'string', 'Favicon de la plataforma en base64'),
    ('platform_support_email', 'soporte@schoolccb.com', 'string', 'Email de soporte'),
    ('platform_sales_email', 'ventas@schoolccb.com', 'string', 'Email de ventas'),
    ('smtp_enabled', 'false', 'boolean', 'SMTP habilitado para emails'),
    ('smtp_host', '', 'string', 'Host SMTP'),
    ('smtp_from_email', '', 'string', 'Email de origen para SMTP'),
    ('stripe_enabled', 'false', 'boolean', 'Stripe habilitado para pagos')
ON CONFLICT (setting_key) DO NOTHING;

-- Índice para búsquedas por key
CREATE INDEX IF NOT EXISTS idx_platform_settings_key ON platform_settings(setting_key);
