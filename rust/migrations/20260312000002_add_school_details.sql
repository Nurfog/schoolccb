-- ============================================
-- Migración: Campos Adicionales para Colegios
-- ============================================

-- Agregar campos de dirección y ubicación a schools
ALTER TABLE schools 
ADD COLUMN IF NOT EXISTS address VARCHAR(255),
ADD COLUMN IF NOT EXISTS comuna VARCHAR(100),
ADD COLUMN IF NOT EXISTS provincia VARCHAR(100),
ADD COLUMN IF NOT EXISTS estado VARCHAR(100),
ADD COLUMN IF NOT EXISTS ciudad VARCHAR(100),
ADD COLUMN IF NOT EXISTS codigo_postal VARCHAR(20),
ADD COLUMN IF NOT EXISTS telefono VARCHAR(50),
ADD COLUMN IF NOT EXISTS email_contacto VARCHAR(255),
ADD COLUMN IF NOT EXISTS sitio_web VARCHAR(255),
ADD COLUMN IF NOT EXISTS rut VARCHAR(50),
ADD COLUMN IF NOT EXISTS razon_social VARCHAR(255);

-- Tabla para representantes legales
CREATE TABLE IF NOT EXISTS legal_representatives (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    school_id UUID REFERENCES schools(id) ON DELETE CASCADE,
    nombre_completo VARCHAR(255) NOT NULL,
    rut VARCHAR(50) NOT NULL,
    cargo VARCHAR(100) NOT NULL, -- Ej: "Representante Legal", "Apoderado", "Director"
    email VARCHAR(255),
    telefono VARCHAR(50),
    direccion VARCHAR(255),
    es_principal BOOLEAN DEFAULT TRUE, -- TRUE = representante principal
    fecha_nombramiento DATE,
    poder_notarial VARCHAR(255), -- Número de escritura o poder
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Índices para búsquedas
CREATE INDEX IF NOT EXISTS idx_schools_ciudad ON schools(ciudad);
CREATE INDEX IF NOT EXISTS idx_schools_comuna ON schools(comuna);
CREATE INDEX IF NOT EXISTS idx_schools_rut ON schools(rut);
CREATE INDEX IF NOT EXISTS idx_legal_representatives_school ON legal_representatives(school_id);
CREATE INDEX IF NOT EXISTS idx_legal_representatives_rut ON legal_representatives(rut);

-- Comentarios
COMMENT ON COLUMN schools.address IS 'Calle y número';
COMMENT ON COLUMN schools.comuna IS 'Comuna o distrito';
COMMENT ON COLUMN schools.provincia IS 'Provincia o región';
COMMENT ON COLUMN schools.estado IS 'Estado o departamento';
COMMENT ON COLUMN schools.ciudad IS 'Ciudad principal';
COMMENT ON COLUMN schools.codigo_postal IS 'Código postal';
COMMENT ON COLUMN schools.telefono IS 'Teléfono de contacto';
COMMENT ON COLUMN schools.email_contacto IS 'Email de contacto institucional';
COMMENT ON COLUMN schools.sitio_web IS 'Sitio web del colegio';
COMMENT ON COLUMN schools.rut IS 'RUT o identificación tributaria';
COMMENT ON COLUMN schools.razon_social IS 'Razón social oficial';

COMMENT ON TABLE legal_representatives IS 'Representantes legales del colegio';
COMMENT ON COLUMN legal_representatives.nombre_completo IS 'Nombre completo del representante';
COMMENT ON COLUMN legal_representatives.rut IS 'RUT o identificación del representante';
COMMENT ON COLUMN legal_representatives.cargo IS 'Cargo o tipo de representación';
COMMENT ON COLUMN legal_representatives.es_principal IS 'Indica si es el representante principal';
COMMENT ON COLUMN legal_representatives.fecha_nombramiento IS 'Fecha de nombramiento como representante';
COMMENT ON COLUMN legal_representatives.poder_notarial IS 'Número de escritura o poder notarial';
