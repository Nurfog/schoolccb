-- ============================================
-- Migración: Agregar columnas resource y action a permissions
-- Fecha: 2026-03-30
-- Descripción: Agrega las columnas resource y action para RBAC granular
-- ============================================

ALTER TABLE permissions ADD COLUMN IF NOT EXISTS resource VARCHAR(100);
ALTER TABLE permissions ADD COLUMN IF NOT EXISTS action VARCHAR(50);

-- ============================================
-- Fin de la Migración
-- ============================================
