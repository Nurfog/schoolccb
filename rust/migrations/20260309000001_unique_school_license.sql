-- Añadir restricción única para asegurar una licencia por colegio (necesario para upsert)
ALTER TABLE saas_licenses ADD CONSTRAINT saas_licenses_school_id_key UNIQUE (school_id);
