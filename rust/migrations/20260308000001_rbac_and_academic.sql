-- Tablas de Permisos (RBAC)
CREATE TABLE IF NOT EXISTS permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL, -- 'users:read', 'users:write', 'academic:manage'
    description TEXT
);

CREATE TABLE IF NOT EXISTS role_permissions (
    role_id INTEGER REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INTEGER REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Módulo Académico: Profesores
CREATE TABLE IF NOT EXISTS teachers (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    bio TEXT,
    specialty VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Módulo Académico: Estudiantes
CREATE TABLE IF NOT EXISTS students (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    enrollment_number VARCHAR(50) UNIQUE,
    parent_id UUID REFERENCES users(id) ON DELETE SET NULL, -- Enlaces a usuario con rol 'padre'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Módulo Académico: Cursos
CREATE TABLE IF NOT EXISTS courses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    school_id UUID REFERENCES schools(id) ON DELETE CASCADE,
    teacher_id UUID REFERENCES teachers(user_id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    grade_level VARCHAR(50), -- '6th Grade', '7th Grade', etc.
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Módulo Académico: Matrículas (Join Students & Courses)
CREATE TABLE IF NOT EXISTS enrollments (
    student_id UUID REFERENCES students(user_id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    enrolled_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) DEFAULT 'active', -- 'active', 'completed', 'dropped'
    PRIMARY KEY (student_id, course_id)
);

-- Insertar algunos permisos básicos
INSERT INTO permissions (name, description) VALUES
('users:view', 'Ver lista de usuarios y perfiles'),
('users:manage', 'Crear, editar y eliminar usuarios'),
('academic:view', 'Ver cursos y calificaciones'),
('academic:manage', 'Gestionar cursos, profesores y matrículas'),
('reports:view', 'Ver reportes administrativos y financieros')
ON CONFLICT (name) DO NOTHING;

-- Asignar permisos básicos a roles
-- Admin: Todo
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p WHERE r.name = 'admin'
ON CONFLICT DO NOTHING;

-- Profesor: Academic View & Manage (limitado)
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'profesor' AND p.name IN ('users:view', 'academic:view', 'academic:manage')
ON CONFLICT DO NOTHING;

-- Alumno: Academic View
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p 
WHERE r.name = 'alumno' AND p.name IN ('academic:view')
ON CONFLICT DO NOTHING;
