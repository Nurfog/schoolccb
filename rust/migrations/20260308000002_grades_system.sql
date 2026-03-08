-- Módulo Académico: Calificaciones (Grades)
CREATE TABLE IF NOT EXISTS grades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    course_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL, -- e.g., 'Primer Parcial', 'Tarea 1'
    grade DECIMAL(5, 2) NOT NULL,
    weight DECIMAL(5, 2) DEFAULT 100.00, -- Porcentaje del total
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- Clave foránea compuesta hacia enrollments
    FOREIGN KEY (student_id, course_id) REFERENCES enrollments(student_id, course_id) ON DELETE CASCADE
);

-- Permisos adicionales para calificaciones
INSERT INTO permissions (name, description) VALUES
('grades:view', 'Ver calificaciones propias o de alumnos'),
('grades:manage', 'Registrar y editar calificaciones')
ON CONFLICT (name) DO NOTHING;

-- Asignar permisos a roles
-- Admin & Profesor pueden gestionar notas
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name IN ('admin', 'profesor') AND p.name IN ('grades:view', 'grades:manage')
ON CONFLICT DO NOTHING;

-- Alumno solo puede ver notas
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'alumno' AND p.name = 'grades:view'
ON CONFLICT DO NOTHING;
