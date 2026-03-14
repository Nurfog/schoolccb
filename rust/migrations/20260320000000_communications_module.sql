-- ============================================
-- Migración: Módulo de Comunicaciones
-- Fecha: 2026-03-20
-- Descripción: Tablas para notificaciones, comunicados,
--              preferencias, plantillas y justificaciones
-- Fase: 6.1 - Infraestructura Base
-- ============================================

-- ============================================
-- 1. Tabla de Notificaciones
-- ============================================
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50) NOT NULL DEFAULT 'info', -- info, warning, error, success, academic, financial
    data JSONB, -- Datos adicionales (ej: { "course_id": "...", "grade": 5.0 })
    categories JSONB DEFAULT '["general"]'::jsonb, -- Categorías para filtrado: ["academic", "financial", ...]
    is_read BOOLEAN DEFAULT FALSE,
    read_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para notificaciones
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_is_read ON notifications(is_read);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
CREATE INDEX IF NOT EXISTS idx_notifications_type ON notifications(type);
CREATE INDEX IF NOT EXISTS idx_notifications_categories ON notifications USING GIN (categories);
CREATE INDEX IF NOT EXISTS idx_notifications_user_unread ON notifications(user_id, is_read) WHERE is_read = FALSE;

-- Comentario
COMMENT ON TABLE notifications IS 'Notificaciones in-app para usuarios';
COMMENT ON COLUMN notifications.data IS 'Datos contextuales en formato JSON para acciones o enlaces';
COMMENT ON COLUMN notifications.categories IS 'Categorías para filtrado: ["academic", "financial", ...]';

-- ============================================
-- 2. Tabla de Preferencias de Notificación
-- ============================================
CREATE TABLE IF NOT EXISTS notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    email_enabled BOOLEAN DEFAULT TRUE,
    push_enabled BOOLEAN DEFAULT TRUE,
    sms_enabled BOOLEAN DEFAULT FALSE,
    in_app_enabled BOOLEAN DEFAULT TRUE,
    categories JSONB DEFAULT '{"academic": true, "financial": true, "administrative": true, "urgent": true}'::jsonb,
    quiet_hours_enabled BOOLEAN DEFAULT FALSE,
    quiet_hours_start TIME DEFAULT '22:00:00',
    quiet_hours_end TIME DEFAULT '07:00:00',
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id)
);

-- Índice para búsqueda rápida
CREATE INDEX IF NOT EXISTS idx_notification_preferences_user_id ON notification_preferences(user_id);

-- Comentario
COMMENT ON TABLE notification_preferences IS 'Preferencias de notificación por usuario';
COMMENT ON COLUMN notification_preferences.categories IS 'Configuración por categoría: {"academic": true, "financial": false, ...}';

-- ============================================
-- 3. Tabla de Plantillas de Notificación
-- ============================================
CREATE TABLE IF NOT EXISTS notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID REFERENCES schools(id) ON DELETE CASCADE, -- NULL = plantilla global del sistema
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE, -- Código identificador (ej: "WELCOME_EMAIL", "GRADE_PUBLISHED")
    subject VARCHAR(255) NOT NULL, -- Asunto para email
    body TEXT NOT NULL, -- Cuerpo del mensaje (con variables {{variable}})
    variables JSONB DEFAULT '[]'::jsonb, -- Lista de variables: ["{{student_name}}", "{{date}}"]
    category VARCHAR(50) NOT NULL DEFAULT 'general', -- academic, financial, administrative, marketing
    channel VARCHAR(50) NOT NULL DEFAULT 'email', -- email, sms, push, in_app
    is_active BOOLEAN DEFAULT TRUE,
    is_system BOOLEAN DEFAULT FALSE, -- true = no editable por colegios
    version INTEGER DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para plantillas
CREATE INDEX IF NOT EXISTS idx_notification_templates_code ON notification_templates(code);
CREATE INDEX IF NOT EXISTS idx_notification_templates_school_id ON notification_templates(school_id);
CREATE INDEX IF NOT EXISTS idx_notification_templates_category ON notification_templates(category);
CREATE INDEX IF NOT EXISTS idx_notification_templates_active ON notification_templates(is_active) WHERE is_active = TRUE;

-- Comentario
COMMENT ON TABLE notification_templates IS 'Plantillas de notificaciones con variables dinámicas';
COMMENT ON COLUMN notification_templates.body IS 'Contenido con sintaxis Handlebars/Tera: {{student_name}}';

-- ============================================
-- 4. Tabla de Comunicados Escolares
-- ============================================
CREATE TABLE IF NOT EXISTS announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    summary VARCHAR(500), -- Resumen corto para vistas previas
    category VARCHAR(50) NOT NULL DEFAULT 'informative', -- urgent, informative, academic, administrative
    target_audience JSONB NOT NULL DEFAULT '{"all": true}'::jsonb, -- {"all": true} o {"grades": [1, 2, 3], "roles": ["parent"]}
    priority INTEGER DEFAULT 1, -- 1=normal, 2=high, 3=critical
    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    scheduled_at TIMESTAMPTZ, -- Programar publicación futura
    expires_at TIMESTAMPTZ, -- Expiración automática
    allow_comments BOOLEAN DEFAULT FALSE,
    requires_confirmation BOOLEAN DEFAULT FALSE, -- Requiere confirmación de lectura
    attachment_urls JSONB DEFAULT '[]'::jsonb, -- Array de URLs de adjuntos
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para comunicados
CREATE INDEX IF NOT EXISTS idx_announcements_school_id ON announcements(school_id);
CREATE INDEX IF NOT EXISTS idx_announcements_category ON announcements(category);
CREATE INDEX IF NOT EXISTS idx_announcements_published ON announcements(is_published) WHERE is_published = TRUE;
CREATE INDEX IF NOT EXISTS idx_announcements_scheduled ON announcements(scheduled_at) WHERE scheduled_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_announcements_created_at ON announcements(created_at DESC);

-- Comentario
COMMENT ON TABLE announcements IS 'Comunicados escolares (circulares, noticias, avisos)';
COMMENT ON COLUMN announcements.target_audience IS 'Audiencia objetivo: {"all": true} o {"grades": [1,2], "roles": ["parent"]}';

-- ============================================
-- 5. Tabla de Lecturas de Comunicados
-- ============================================
CREATE TABLE IF NOT EXISTS announcement_readings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_id UUID NOT NULL REFERENCES announcements(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_confirmed BOOLEAN DEFAULT FALSE, -- Confirmación explícita (para requires_confirmation)
    read_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    ip_address INET,
    user_agent TEXT,
    UNIQUE(announcement_id, user_id)
);

-- Índices para lecturas
CREATE INDEX IF NOT EXISTS idx_announcement_readings_announcement_id ON announcement_readings(announcement_id);
CREATE INDEX IF NOT EXISTS idx_announcement_readings_user_id ON announcement_readings(user_id);
CREATE INDEX IF NOT EXISTS idx_announcement_readings_confirmed ON announcement_readings(announcement_id, is_confirmed);

-- Comentario
COMMENT ON TABLE announcement_readings IS 'Registro de lecturas y confirmaciones de comunicados';

-- ============================================
-- 6. Tabla de Justificaciones de Inasistencia
-- ============================================
CREATE TABLE IF NOT EXISTS attendance_justifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    absence_date DATE NOT NULL,
    absence_type VARCHAR(50) DEFAULT 'full_day', -- full_day, partial, late, early_departure
    start_time TIME, -- Para ausencias parciales
    end_time TIME, -- Para ausencias parciales
    reason TEXT NOT NULL,
    attachment_urls JSONB DEFAULT '[]'::jsonb, -- URLs de justificantes adjuntos
    status VARCHAR(50) DEFAULT 'pending', -- pending, approved, rejected, cancelled
    reviewed_by UUID REFERENCES users(id), -- Usuario que revisó (profesor/admin)
    review_notes TEXT, -- Comentarios de la revisión
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices para justificaciones
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_student_id ON attendance_justifications(student_id);
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_parent_id ON attendance_justifications(parent_id);
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_school_id ON attendance_justifications(school_id);
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_status ON attendance_justifications(status);
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_date ON attendance_justifications(absence_date DESC);
CREATE INDEX IF NOT EXISTS idx_attendance_justifications_pending ON attendance_justifications(school_id, status) WHERE status = 'pending';

-- Comentario
COMMENT ON TABLE attendance_justifications IS 'Justificaciones de inasistencia enviadas por padres';
COMMENT ON COLUMN attendance_justifications.absence_type IS 'Tipo de ausencia: full_day, partial, late, early_departure';

-- ============================================
-- 7. Vista de Resumen de Comunicados
-- ============================================
CREATE OR REPLACE VIEW announcement_stats AS
SELECT 
    a.id AS announcement_id,
    a.title,
    a.school_id,
    a.category,
    a.is_published,
    a.requires_confirmation,
    COUNT(DISTINCT ar.user_id) AS total_read,
    COUNT(DISTINCT CASE WHEN ar.is_confirmed THEN ar.user_id END) AS total_confirmed,
    CASE 
        WHEN a.requires_confirmation THEN 
            ROUND(COUNT(DISTINCT CASE WHEN ar.is_confirmed THEN ar.user_id END)::numeric / 
                  NULLIF(COUNT(DISTINCT ar.user_id), 0)::numeric * 100, 2)
        ELSE NULL
    END AS confirmation_percentage
FROM announcements a
LEFT JOIN announcement_readings ar ON a.id = ar.announcement_id
GROUP BY a.id, a.title, a.school_id, a.category, a.is_published, a.requires_confirmation;

-- ============================================
-- 8. Funciones Utilitarias
-- ============================================

-- Función para contar notificaciones no leídas
CREATE OR REPLACE FUNCTION count_unread_notifications(p_user_id UUID)
RETURNS INTEGER AS $$
BEGIN
    RETURN (
        SELECT COUNT(*)::INTEGER
        FROM notifications
        WHERE user_id = p_user_id
          AND is_read = FALSE
          AND (expires_at IS NULL OR expires_at > NOW())
    );
END;
$$ LANGUAGE plpgsql STABLE;

-- Función para marcar todas las notificaciones como leídas
CREATE OR REPLACE FUNCTION mark_all_notifications_read(p_user_id UUID)
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE notifications
    SET is_read = TRUE,
        read_at = CURRENT_TIMESTAMP
    WHERE user_id = p_user_id
      AND is_read = FALSE;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Función para obtener preferencias de un usuario (crea si no existe)
CREATE OR REPLACE FUNCTION get_or_create_notification_preferences(p_user_id UUID)
RETURNS SETOF notification_preferences AS $$
BEGIN
    -- Insertar si no existe
    INSERT INTO notification_preferences (user_id)
    VALUES (p_user_id)
    ON CONFLICT (user_id) DO NOTHING;
    
    RETURN QUERY
    SELECT * FROM notification_preferences
    WHERE user_id = p_user_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 9. Trigger para actualizar updated_at
-- ============================================

-- Trigger para notification_templates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_notification_templates_updated_at
    BEFORE UPDATE ON notification_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_announcements_updated_at
    BEFORE UPDATE ON announcements
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_attendance_justifications_updated_at
    BEFORE UPDATE ON attendance_justifications
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_notification_preferences_updated_at
    BEFORE UPDATE ON notification_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 10. Datos Iniciales (Plantillas del Sistema)
-- ============================================

-- Plantilla: Bienvenida
INSERT INTO notification_templates (code, name, subject, body, variables, category, channel, is_system, is_active)
VALUES (
    'WELCOME_EMAIL',
    'Email de Bienvenida',
    '¡Bienvenido a {{school_name}}!',
    'Hola {{user_name}},\n\nTe damos la bienvenida a {{school_name}}.\n\nTu cuenta ha sido creada exitosamente.\n\nSaludos,\nEl equipo de {{school_name}}',
    '["{{user_name}}", "{{school_name}}"]'::jsonb,
    'administrative',
    'email',
    TRUE,
    TRUE
) ON CONFLICT (code) DO NOTHING;

-- Plantilla: Nueva Calificación
INSERT INTO notification_templates (code, name, subject, body, variables, category, channel, is_system, is_active)
VALUES (
    'GRADE_PUBLISHED',
    'Nueva Calificación Publicada',
    'Nueva calificación en {{course_name}}',
    'Hola {{student_name}},\n\nSe ha publicado una nueva calificación:\n\nMateria: {{evaluation_name}}\nCurso: {{course_name}}\nCalificación: {{grade}}\n\nRevisa tu boletín para más detalles.',
    '["{{student_name}}", "{{course_name}}", "{{evaluation_name}}", "{{grade}}"]'::jsonb,
    'academic',
    'email',
    TRUE,
    TRUE
) ON CONFLICT (code) DO NOTHING;

-- Plantilla: Ausencia Registrada
INSERT INTO notification_templates (code, name, subject, body, variables, category, channel, is_system, is_active)
VALUES (
    'ABSENCE_RECORDED',
    'Ausencia Registrada',
    'Ausencia registrada - {{date}}',
    'Estimado {{parent_name}},\n\nLe informamos que se registró una ausencia para {{student_name}} el día {{date}}.\n\nSi necesita justificarla, puede hacerlo desde el portal de padres.',
    '["{{parent_name}}", "{{student_name}}", "{{date}}"]'::jsonb,
    'academic',
    'email',
    TRUE,
    TRUE
) ON CONFLICT (code) DO NOTHING;

-- Plantilla: Comunicado Urgente
INSERT INTO notification_templates (code, name, subject, body, variables, category, channel, is_system, is_active)
VALUES (
    'URGENT_ANNOUNCEMENT',
    'Comunicado Urgente',
    'URGENTE: {{title}}',
    '{{content}}\n\nPor favor, tome nota de esta información importante.',
    '["{{title}}", "{{content}}"]'::jsonb,
    'urgent',
    'email',
    TRUE,
    TRUE
) ON CONFLICT (code) DO NOTHING;

-- ============================================
-- Fin de la Migración
-- ============================================
