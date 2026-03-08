-- Extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabla de Colegios (Multi-tenancy)
CREATE TABLE IF NOT EXISTS schools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de Roles
CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL, -- 'admin', 'profesor', 'alumno', 'padre'
    description TEXT
);

-- Tabla de Usuarios
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    school_id UUID REFERENCES schools(id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES roles(id),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar Roles por defecto
INSERT INTO roles (name, description) VALUES
('admin', 'Administrador del sistema o del colegio'),
('profesor', 'Docente encargado de impartir materias'),
('alumno', 'Estudiante de la institución'),
('padre', 'Padre o tutor legal del alumno')
ON CONFLICT (name) DO NOTHING;

-- Insertar Colegio de Prueba
INSERT INTO schools (name, subdomain) 
VALUES ('Colegio Central de Bogotá', 'ccb')
ON CONFLICT (subdomain) DO NOTHING;

-- Insertar Usuario Administrador de Prueba (password: admin123)
-- Nota: En producción las contraseñas DEBEN estar hasheadas correctamente.
INSERT INTO users (school_id, role_id, name, email, password_hash)
SELECT id, (SELECT id FROM roles WHERE name = 'admin'), 'Admin CCB', 'admin@ccb.edu.co', '$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$i769B7jI77yXqV6N7z6w7w' 
FROM schools WHERE subdomain = 'ccb'
LIMIT 1
ON CONFLICT (email) DO NOTHING;
