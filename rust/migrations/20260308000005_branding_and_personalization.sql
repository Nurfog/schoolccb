-- Phase 19: Personalización y Marca Blanca
-- Agregar columnas de branding a la tabla de colegios

ALTER TABLE schools 
ADD COLUMN IF NOT EXISTS logo_url TEXT,
ADD COLUMN IF NOT EXISTS primary_color VARCHAR(20),
ADD COLUMN IF NOT EXISTS secondary_color VARCHAR(20);
