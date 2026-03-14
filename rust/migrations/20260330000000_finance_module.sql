-- ============================================
-- Migración: Módulo Financiero (Fase 8)
-- Fecha: 2026-03-30
-- Descripción: Gestión financiera, pagos, pensiones, facturas
-- Fase: 8.1 - Finanzas
-- ============================================

-- ============================================
-- 1. Períodos Financieros
-- ============================================
CREATE TABLE IF NOT EXISTS financial_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL, -- "Año Fiscal 2026", "Semestre 1 2026"
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    is_closed BOOLEAN DEFAULT FALSE, -- Período cerrado (no se pueden hacer cambios)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMPTZ,
    UNIQUE(school_id, name)
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_financial_periods_school_id ON financial_periods(school_id);
CREATE INDEX IF NOT EXISTS idx_financial_periods_active ON financial_periods(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE financial_periods IS 'Períodos fiscales para organización financiera';

-- ============================================
-- 2. Conceptos de Pago / Rubros
-- ============================================
CREATE TABLE IF NOT EXISTS payment_concepts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    name VARCHAR(150) NOT NULL, -- "Pensión Mensual", "Matrícula", "Uniforme", "Libros"
    code VARCHAR(50) UNIQUE, -- Código contable
    category VARCHAR(50) NOT NULL DEFAULT 'pension', -- pension, enrollment, uniform, books, service, other
    amount DECIMAL(10,2) NOT NULL DEFAULT 0, -- Monto base
    is_recurring BOOLEAN DEFAULT TRUE, -- Se repite cada mes
    frequency_months INTEGER DEFAULT 1, -- Cada cuántos meses se cobra (1 = mensual)
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_payment_concepts_school_id ON payment_concepts(school_id);
CREATE INDEX IF NOT EXISTS idx_payment_concepts_category ON payment_concepts(category);
CREATE INDEX IF NOT EXISTS idx_payment_concepts_active ON payment_concepts(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE payment_concepts IS 'Conceptos o rubros que se pueden cobrar (pensiones, matrícula, etc.)';

-- ============================================
-- 3. Pensiones / Cuotas Mensuales
-- ============================================
CREATE TABLE IF NOT EXISTS pensions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    financial_period_id UUID REFERENCES financial_periods(id),
    payment_concept_id UUID REFERENCES payment_concepts(id),
    month INTEGER NOT NULL CHECK (month BETWEEN 1 AND 12),
    year INTEGER NOT NULL CHECK (year >= 2020),
    amount DECIMAL(10,2) NOT NULL, -- Monto total de la pensión
    discount DECIMAL(10,2) DEFAULT 0, -- Descuentos aplicados
    surcharge DECIMAL(10,2) DEFAULT 0, -- Recargos por mora
    total DECIMAL(10,2) NOT NULL, -- amount - discount + surcharge
    due_date DATE NOT NULL, -- Fecha límite de pago
    status VARCHAR(50) DEFAULT 'pending', -- pending, paid, partial, overdue, cancelled, forgiven
    paid_amount DECIMAL(10,2) DEFAULT 0, -- Monto ya pagado
    paid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(student_id, month, year)
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_pensions_school_id ON pensions(school_id);
CREATE INDEX IF NOT EXISTS idx_pensions_student_id ON pensions(student_id);
CREATE INDEX IF NOT EXISTS idx_pensions_period ON pensions(financial_period_id);
CREATE INDEX IF NOT EXISTS idx_pensions_status ON pensions(status);
CREATE INDEX IF NOT EXISTS idx_pensions_due_date ON pensions(due_date);
CREATE INDEX IF NOT EXISTS idx_pensions_overdue ON pensions(status, due_date) WHERE status = 'overdue';
CREATE INDEX IF NOT EXISTS idx_pensions_pending ON pensions(status) WHERE status IN ('pending', 'partial', 'overdue');

COMMENT ON TABLE pensions IS 'Pensiones mensuales de estudiantes';

-- ============================================
-- 4. Pagos
-- ============================================
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    payer_id UUID REFERENCES users(id), -- Usuario que realiza el pago (padre/responsable)
    payment_method VARCHAR(50) DEFAULT 'cash', -- cash, card, transfer, stripe, paypal, check
    payment_reference VARCHAR(100), -- Número de transacción, cheque, etc.
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',
    notes TEXT,
    status VARCHAR(50) DEFAULT 'completed', -- completed, pending, cancelled, refunded
    stripe_payment_intent_id VARCHAR(100), -- ID de Stripe si aplica
    stripe_charge_id VARCHAR(100),
    processed_by UUID REFERENCES users(id), -- Usuario que procesó el pago
    processed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_payments_school_id ON payments(school_id);
CREATE INDEX IF NOT EXISTS idx_payments_student_id ON payments(student_id);
CREATE INDEX IF NOT EXISTS idx_payments_payer_id ON payments(payer_id);
CREATE INDEX IF NOT EXISTS idx_payments_method ON payments(payment_method);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_payments_stripe ON payments(stripe_payment_intent_id) WHERE stripe_payment_intent_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_payments_created_at ON payments(created_at DESC);

COMMENT ON TABLE payments IS 'Pagos realizados por estudiantes/padres';

-- ============================================
-- 5. Detalle de Aplicación de Pagos
-- ============================================
CREATE TABLE IF NOT EXISTS payment_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    pension_id UUID REFERENCES pensions(id) ON DELETE CASCADE, -- A qué pensión se aplica
    amount DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(payment_id, pension_id)
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_payment_applications_payment_id ON payment_applications(payment_id);
CREATE INDEX IF NOT EXISTS idx_payment_applications_pension_id ON payment_applications(pension_id);

COMMENT ON TABLE payment_applications IS 'Cómo se aplican los pagos a pensiones específicas';

-- ============================================
-- 6. Becas y Descuentos
-- ============================================
CREATE TABLE IF NOT EXISTS scholarships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(150) NOT NULL, -- "Beca por Rendimiento", "Descuento por Hermanos"
    type VARCHAR(50) DEFAULT 'percentage', -- percentage, fixed
    value DECIMAL(10,2) NOT NULL, -- Porcentaje (0-100) o monto fijo
    start_date DATE NOT NULL,
    end_date DATE, -- NULL = indefinido
    is_active BOOLEAN DEFAULT TRUE,
    reason TEXT,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_scholarships_school_id ON scholarships(school_id);
CREATE INDEX IF NOT EXISTS idx_scholarships_student_id ON scholarships(student_id);
CREATE INDEX IF NOT EXISTS idx_scholarships_active ON scholarships(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE scholarships IS 'Becas y descuentos aplicados a estudiantes';

-- ============================================
-- 7. Facturas / Recibos
-- ============================================
CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    school_id UUID NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    invoice_number VARCHAR(50) UNIQUE NOT NULL, -- "FAC-2026-00001"
    invoice_type VARCHAR(50) DEFAULT 'receipt', -- receipt, invoice, credit_note
    subtotal DECIMAL(10,2) NOT NULL,
    tax DECIMAL(10,2) DEFAULT 0,
    total DECIMAL(10,2) NOT NULL,
    status VARCHAR(50) DEFAULT 'issued', -- issued, paid, cancelled, void
    issued_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    paid_at TIMESTAMPTZ,
    pdf_url VARCHAR(500), -- URL del PDF generado
    stripe_invoice_id VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_invoices_school_id ON invoices(school_id);
CREATE INDEX IF NOT EXISTS idx_invoices_student_id ON invoices(student_id);
CREATE INDEX IF NOT EXISTS idx_invoices_number ON invoices(invoice_number);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_issued_at ON invoices(issued_at DESC);

COMMENT ON TABLE invoices IS 'Facturas y recibos generados';

-- ============================================
-- 8. Items de Factura
-- ============================================
CREATE TABLE IF NOT EXISTS invoice_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    payment_concept_id UUID REFERENCES payment_concepts(id),
    description TEXT NOT NULL,
    quantity INTEGER DEFAULT 1,
    unit_price DECIMAL(10,2) NOT NULL,
    amount DECIMAL(10,2) NOT NULL, -- quantity * unit_price
    pension_id UUID REFERENCES pensions(id), -- Vinculado a pensión específica
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice_id ON invoice_items(invoice_id);
CREATE INDEX IF NOT EXISTS idx_invoice_items_concept_id ON invoice_items(payment_concept_id);

COMMENT ON TABLE invoice_items IS 'Detalle de items en una factura';

-- ============================================
-- 9. Morosidad y Recordatorios
-- ============================================
CREATE TABLE IF NOT EXISTS payment_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pension_id UUID NOT NULL REFERENCES pensions(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reminder_type VARCHAR(50) DEFAULT 'email', -- email, sms, push
    status VARCHAR(50) DEFAULT 'pending', -- pending, sent, failed
    sent_at TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_payment_reminders_pension_id ON payment_reminders(pension_id);
CREATE INDEX IF NOT EXISTS idx_payment_reminders_student_id ON payment_reminders(student_id);
CREATE INDEX IF NOT EXISTS idx_payment_reminders_status ON payment_reminders(status);

COMMENT ON TABLE payment_reminders IS 'Recordatorios de pago enviados';

-- ============================================
-- 10. Vistas Financieras
-- ============================================

-- Vista: Resumen financiero por estudiante
CREATE OR REPLACE VIEW student_financial_summary AS
SELECT 
    s.id AS student_id,
    s.name AS student_name,
    sc.id AS school_id,
    sc.name AS school_name,
    COUNT(DISTINCT p.id) AS total_pensions,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 'paid') AS paid_pensions,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status IN ('pending', 'partial')) AS pending_pensions,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 'overdue') AS overdue_pensions,
    SUM(p.total) FILTER (WHERE p.status != 'cancelled') AS total_amount,
    SUM(p.paid_amount) FILTER (WHERE p.status != 'cancelled') AS paid_amount,
    SUM(p.total - p.paid_amount) FILTER (WHERE p.status != 'cancelled') AS outstanding_balance,
    SUM(p.total) FILTER (WHERE p.status = 'overdue') AS overdue_amount
FROM users s
JOIN schools sc ON s.school_id = sc.id
LEFT JOIN pensions p ON s.id = p.student_id
WHERE s.role_id = (SELECT id FROM roles WHERE name = 'alumno')
GROUP BY s.id, s.name, sc.id, sc.name;

-- Vista: Ingresos mensuales por colegio
CREATE OR REPLACE VIEW monthly_revenue_by_school AS
SELECT 
    sc.id AS school_id,
    sc.name AS school_name,
    DATE_TRUNC('month', pa.processed_at) AS month,
    COUNT(DISTINCT pa.id) AS payment_count,
    SUM(pa.amount) AS total_revenue,
    SUM(pa.amount) FILTER (WHERE pa.payment_method = 'stripe') AS stripe_revenue,
    SUM(pa.amount) FILTER (WHERE pa.payment_method = 'cash') AS cash_revenue,
    SUM(pa.amount) FILTER (WHERE pa.payment_method = 'card') AS card_revenue
FROM schools sc
JOIN payments pa ON sc.id = pa.school_id
WHERE pa.status = 'completed'
GROUP BY sc.id, sc.name, DATE_TRUNC('month', pa.processed_at)
ORDER BY month DESC;

-- Vista: Pensiones vencidas por cobrar
CREATE OR REPLACE VIEW overdue_pensions_view AS
SELECT 
    p.id AS pension_id,
    p.student_id,
    s.name AS student_name,
    s.email AS student_email,
    p.month,
    p.year,
    p.total,
    p.paid_amount,
    (p.total - p.paid_amount) AS outstanding_balance,
    p.due_date,
    CURRENT_DATE - p.due_date AS days_overdue,
    sc.name AS school_name
FROM pensions p
JOIN users s ON p.student_id = s.id
JOIN schools sc ON p.school_id = sc.id
WHERE p.status = 'overdue' OR (p.status IN ('pending', 'partial') AND p.due_date < CURRENT_DATE);

-- ============================================
-- 11. Funciones Financieras
-- ============================================

-- Función para actualizar estado de pensión basado en pagos
CREATE OR REPLACE FUNCTION update_pension_status()
RETURNS TRIGGER AS $$
BEGIN
    -- Calcular monto pagado
    UPDATE pensions
    SET paid_amount = (
            SELECT COALESCE(SUM(pa.amount), 0)
            FROM payment_applications pa
            WHERE pa.pension_id = pensions.id
        ),
        status = CASE
            WHEN (SELECT COALESCE(SUM(pa.amount), 0) FROM payment_applications pa WHERE pa.pension_id = pensions.id) >= pensions.total THEN 'paid'
            WHEN (SELECT COALESCE(SUM(pa.amount), 0) FROM payment_applications pa WHERE pa.pension_id = pensions.id) > 0 THEN 'partial'
            WHEN pensions.due_date < CURRENT_DATE THEN 'overdue'
            ELSE 'pending'
        END,
        paid_at = CASE
            WHEN (SELECT COALESCE(SUM(pa.amount), 0) FROM payment_applications pa WHERE pa.pension_id = pensions.id) >= pensions.total
            THEN CURRENT_TIMESTAMP
            ELSE NULL
        END,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.pension_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger para actualizar pensión cuando se aplica pago
CREATE TRIGGER trg_update_pension_on_payment_application
    AFTER INSERT OR UPDATE ON payment_applications
    FOR EACH ROW EXECUTE FUNCTION update_pension_status();

-- Función para generar número de factura
CREATE OR REPLACE FUNCTION generate_invoice_number(p_school_id UUID)
RETURNS VARCHAR(50) AS $$
DECLARE
    v_year INTEGER;
    v_sequence INTEGER;
    v_invoice_number VARCHAR(50);
BEGIN
    v_year := EXTRACT(YEAR FROM CURRENT_DATE)::INTEGER;

    -- Obtener siguiente secuencia
    SELECT COALESCE(MAX(
        SUBSTRING(invoice_number FROM 'FAC-[0-9]{4}-([0-9]+)$')::INTEGER
    ), 0) + 1
    INTO v_sequence
    FROM invoices
    WHERE school_id = p_school_id AND invoice_number LIKE 'FAC-' || v_year || '-%';

    v_invoice_number := 'FAC-' || v_year || '-' || LPAD(v_sequence::TEXT, 5, '0');

    RETURN v_invoice_number;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 12. Datos Iniciales
-- ============================================

-- Conceptos de pago por defecto (se pueden personalizar por colegio)
INSERT INTO payment_concepts (school_id, name, code, category, amount, is_recurring, frequency_months)
SELECT 
    id,
    'Pensión Mensual',
    'PENSION-001',
    'pension',
    100.00,
    TRUE,
    1
FROM schools
ON CONFLICT (code) DO NOTHING;

INSERT INTO payment_concepts (school_id, name, code, category, amount, is_recurring, frequency_months)
SELECT 
    id,
    'Matrícula',
    'ENROLLMENT-001',
    'enrollment',
    200.00,
    FALSE,
    12
FROM schools
ON CONFLICT (code) DO NOTHING;

-- ============================================
-- Fin de la Migración
-- ============================================
