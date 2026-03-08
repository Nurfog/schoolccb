-- Módulo Académico: Periodos y Asistencia
CREATE TABLE IF NOT EXISTS academic_periods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL, -- e.g., 'Primer Bimestre 2026'
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS attendance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    course_id UUID NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'present', -- present, absent, late, justified
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- Clave foránea hacia enrollments
    FOREIGN KEY (student_id, course_id) REFERENCES enrollments(student_id, course_id) ON DELETE CASCADE,
    -- Un registro de asistencia por alumno/curso/día
    UNIQUE (student_id, course_id, date)
);

-- Actualizar grades para referenciar periodos
ALTER TABLE grades ADD COLUMN IF NOT EXISTS period_id UUID REFERENCES academic_periods(id) ON DELETE SET NULL;

-- Sembrar un periodo inicial para pruebas
INSERT INTO academic_periods (school_id, name, start_date, end_date, is_active)
SELECT id, 'Primer Bimestre 2026', '2026-01-20', '2026-03-31', true
FROM schools LIMIT 1;

-- Permisos para asistencia y periodos
INSERT INTO permissions (name, description) VALUES
('periods:manage', 'Crear y activar periodos académicos'),
('attendance:record', 'Pasar lista en clase'),
('attendance:view', 'Ver records de asistencia')
ON CONFLICT (name) DO NOTHING;

-- Asignación de permisos
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'admin' AND p.name IN ('periods:manage', 'attendance:record', 'attendance:view')
ON CONFLICT DO NOTHING;

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'profesor' AND p.name IN ('attendance:record', 'attendance:view')
ON CONFLICT DO NOTHING;

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'alumno' AND p.name = 'attendance:view'
ON CONFLICT DO NOTHING;
