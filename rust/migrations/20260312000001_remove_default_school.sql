-- ============================================
-- Migración: Eliminar colegio por defecto
-- ============================================

-- Eliminar colegio por defecto si existe
DELETE FROM schools WHERE subdomain = 'ccb' AND name = 'Colegio Central de Bogotá';

-- Eliminar usuarios admin por defecto
DELETE FROM users WHERE email = 'admin@ccb.edu.co';

-- Comentario: Ahora el sistema no crea colegios por defecto
-- Cada colegio se crea manualmente desde la Root Console
