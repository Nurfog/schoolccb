-- ============================================
-- Migración: Agregar columna is_system a roles
-- Fecha: 2026-03-30
-- Descripción: Agrega la columna is_system para marcar roles del sistema
-- ============================================

ALTER TABLE roles ADD COLUMN IF NOT EXISTS is_system BOOLEAN DEFAULT FALSE;

-- ============================================
-- Fin de la Migración
-- ============================================
