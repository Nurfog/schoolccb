-- Extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabla de Colegios (Multi-tenancy)
CREATE TABLE IF NOT EXISTS schools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,
    country_id INTEGER, -- REFERENCES countries(id) -- Se agrega después
    is_system_admin BOOLEAN DEFAULT FALSE, -- TRUE para colegio sistema (root)
    logo_url TEXT,
    primary_color VARCHAR(50),
    secondary_color VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de Roles
CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL, -- 'admin', 'profesor', 'alumno', 'padre', 'root'
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
('root', 'Dueño de la plataforma - Acceso total'),
('admin', 'Administrador del sistema o del colegio'),
('profesor', 'Docente encargado de impartir materias'),
('alumno', 'Estudiante de la institución'),
('padre', 'Padre o tutor legal del alumno')
ON CONFLICT (name) DO NOTHING;

-- NO insertar colegio por defecto
-- Cada colegio se crea manualmente desde la sistema
