-- ============================================
-- Migración: Actualización de saas_licenses para planes premium
-- ============================================

-- Asegurar que la columna plan_type tenga los valores correctos
-- Valores válidos: 'basic', 'premium', 'enterprise'

-- Actualizar registros existentes a 'basic' si no tienen un valor válido
UPDATE saas_licenses 
SET plan_type = 'basic' 
WHERE plan_type NOT IN ('basic', 'premium', 'enterprise');

-- Agregar constraint para validar plan_type
ALTER TABLE saas_licenses 
    DROP CONSTRAINT IF EXISTS check_plan_type;

ALTER TABLE saas_licenses 
    ADD CONSTRAINT check_plan_type 
    CHECK (plan_type IN ('basic', 'premium', 'enterprise'));

-- Agregar columna de trial (período de prueba)
ALTER TABLE saas_licenses 
    ADD COLUMN IF NOT EXISTS is_trial BOOLEAN DEFAULT FALSE;

-- Agregar columna para Stripe customer ID
ALTER TABLE saas_licenses 
    ADD COLUMN IF NOT EXISTS stripe_customer_id VARCHAR(255);

-- Agregar columna para Stripe subscription ID
ALTER TABLE saas_licenses 
    ADD COLUMN IF NOT EXISTS stripe_subscription_id VARCHAR(255);

-- Agregar columna para último pago
ALTER TABLE saas_licenses 
    ADD COLUMN IF NOT EXISTS last_payment_date TIMESTAMP WITH TIME ZONE;

-- Agregar columna para monto del último pago
ALTER TABLE saas_licenses 
    ADD COLUMN IF NOT EXISTS last_payment_amount DECIMAL(10, 2);

-- Índices para búsquedas frecuentes
CREATE INDEX IF NOT EXISTS idx_saas_licenses_plan_type ON saas_licenses(plan_type);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_is_trial ON saas_licenses(is_trial);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_stripe_customer ON saas_licenses(stripe_customer_id);

-- Comentario
COMMENT ON COLUMN saas_licenses.plan_type IS 'Tipo de plan: basic ($49), premium ($99), enterprise ($249)';
COMMENT ON COLUMN saas_licenses.is_trial IS 'Indica si es período de prueba de 14 días';
COMMENT ON COLUMN saas_licenses.stripe_customer_id IS 'Stripe Customer ID para facturación';
COMMENT ON COLUMN saas_licenses.stripe_subscription_id IS 'Stripe Subscription ID';
