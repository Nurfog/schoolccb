-- init.sql
-- Script para la inicialización de la base de datos PostgreSQL

-- Conectarse a la base de datos principal (colleges)
\c colleges;

-- Crear la tabla para gestionar los tenants (colegios)
CREATE TABLE IF NOT EXISTS public.tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    schema_name VARCHAR(63) NOT NULL UNIQUE, -- Nombre del esquema para este tenant
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar un tenant de ejemplo si no existe
INSERT INTO public.tenants (name, schema_name)
VALUES ('Colegio Ejemplo', 'tenant_ejemplo')
ON CONFLICT (name) DO NOTHING;

-- Función para crear un esquema para un nuevo tenant
CREATE OR REPLACE FUNCTION create_tenant_schema(tenant_schema_name TEXT)
RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE SCHEMA IF NOT EXISTS %I', tenant_schema_name);
    -- Otorgar permisos al usuario de la aplicación si se define uno
    -- Por ahora, el usuario por defecto 'postgres' tiene todos los permisos.
END;
$$ LANGUAGE plpgsql;

-- Ejecutar la función para el tenant de ejemplo
SELECT create_tenant_schema('tenant_ejemplo');

-- Crear una tabla de ejemplo dentro del esquema del tenant de ejemplo
CREATE TABLE IF NOT EXISTS tenant_ejemplo.students (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    enrollment_date DATE DEFAULT CURRENT_DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar un estudiante de ejemplo
INSERT INTO tenant_ejemplo.students (first_name, last_name)
VALUES ('Juan', 'Perez')
ON CONFLICT (id) DO NOTHING;

-- Crear la tabla de usuarios globales (para login)
CREATE TABLE IF NOT EXISTS public.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES public.tenants(id),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'student', -- admin, teacher, student, parent
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar un usuario de ejemplo para el tenant_ejemplo
INSERT INTO public.users (tenant_id, email, password_hash, role)
VALUES ( (SELECT id FROM public.tenants WHERE name = 'Colegio Ejemplo'), 'admin@ejemplo.com', 'hashed_password_here', 'admin' )
ON CONFLICT (email) DO NOTHING;