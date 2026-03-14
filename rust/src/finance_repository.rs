// ============================================
// Repositorio Financiero (Fase 8)
// ============================================

use crate::models::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct FinanceRepository {
    pool: Pool<Postgres>,
}

impl FinanceRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // ============================================
    // Pensiones
    // ============================================

    /// Crear pensión
    pub async fn create_pension(
        &self,
        school_id: Uuid,
        student_id: Uuid,
        month: i32,
        year: i32,
        amount: rust_decimal::Decimal,
        discount: rust_decimal::Decimal,
        surcharge: rust_decimal::Decimal,
        due_date: chrono::NaiveDate,
    ) -> Result<Pension, sqlx::Error> {
        let total = amount - discount + surcharge;
        
        sqlx::query_as::<_, Pension>(
            r#"
            INSERT INTO pensions (
                school_id, student_id, month, year,
                amount, discount, surcharge, total, due_date
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (student_id, month, year) DO UPDATE
            SET amount = EXCLUDED.amount,
                discount = EXCLUDED.discount,
                surcharge = EXCLUDED.surcharge,
                total = EXCLUDED.total,
                due_date = EXCLUDED.due_date,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(student_id)
        .bind(month)
        .bind(year)
        .bind(amount)
        .bind(discount)
        .bind(surcharge)
        .bind(total)
        .bind(due_date)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener pensiones de un estudiante
    pub async fn get_student_pensions(
        &self,
        student_id: Uuid,
        year: Option<i32>,
    ) -> Result<Vec<PensionWithStudent>, sqlx::Error> {
        let mut query = String::from(
            r#"
            SELECT 
                p.*,
                s.name as student_name,
                s.email as student_email
            FROM pensions p
            JOIN users s ON p.student_id = s.id
            WHERE p.student_id = $1
            "#
        );

        if let Some(y) = year {
            query.push_str(&format!(" AND p.year = {}", y));
        }

        query.push_str(" ORDER BY p.year DESC, p.month DESC");

        sqlx::query_as::<_, PensionWithStudent>(&query)
            .bind(student_id)
            .fetch_all(&self.pool)
            .await
    }

    /// Obtener pensiones vencidas
    pub async fn get_overdue_pensions(
        &self,
        school_id: Uuid,
        limit: i64,
    ) -> Result<Vec<PensionWithStudent>, sqlx::Error> {
        sqlx::query_as::<_, PensionWithStudent>(
            r#"
            SELECT 
                p.*,
                s.name as student_name,
                s.email as student_email
            FROM pensions p
            JOIN users s ON p.student_id = s.id
            WHERE p.school_id = $1
              AND p.status = 'overdue'
            ORDER BY p.due_date ASC
            LIMIT $2
            "#
        )
        .bind(school_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// Actualizar estado de pensión
    pub async fn update_pension_status(
        &self,
        pension_id: Uuid,
        status: &str,
    ) -> Result<Pension, sqlx::Error> {
        sqlx::query_as::<_, Pension>(
            r#"
            UPDATE pensions
            SET status = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(pension_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    // ============================================
    // Pagos
    // ============================================

    /// Registrar pago
    pub async fn create_payment(
        &self,
        school_id: Uuid,
        student_id: Uuid,
        payer_id: Option<Uuid>,
        amount: rust_decimal::Decimal,
        payment_method: &str,
        payment_reference: Option<&str>,
        notes: Option<&str>,
        processed_by: Uuid,
    ) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payments (
                school_id, student_id, payer_id,
                amount, payment_method, payment_reference, notes,
                processed_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(student_id)
        .bind(payer_id)
        .bind(amount)
        .bind(payment_method)
        .bind(payment_reference)
        .bind(notes)
        .bind(processed_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Aplicar pago a pensión
    pub async fn apply_payment_to_pension(
        &self,
        payment_id: Uuid,
        pension_id: Uuid,
        amount: rust_decimal::Decimal,
    ) -> Result<PaymentApplication, sqlx::Error> {
        sqlx::query_as::<_, PaymentApplication>(
            r#"
            INSERT INTO payment_applications (payment_id, pension_id, amount)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(payment_id)
        .bind(pension_id)
        .bind(amount)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener pagos de un estudiante
    pub async fn get_student_payments(
        &self,
        student_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT * FROM payments
            WHERE student_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(student_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    // ============================================
    // Becas
    // ============================================

    /// Crear beca
    pub async fn create_scholarship(
        &self,
        school_id: Uuid,
        student_id: Uuid,
        name: &str,
        type_field: &str,
        value: rust_decimal::Decimal,
        start_date: chrono::NaiveDate,
        end_date: Option<chrono::NaiveDate>,
        reason: Option<&str>,
        approved_by: Uuid,
    ) -> Result<Scholarship, sqlx::Error> {
        sqlx::query_as::<_, Scholarship>(
            r#"
            INSERT INTO scholarships (
                school_id, student_id, name, type_field, value,
                start_date, end_date, reason, approved_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(student_id)
        .bind(name)
        .bind(type_field)
        .bind(value)
        .bind(start_date)
        .bind(end_date)
        .bind(reason)
        .bind(approved_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener becas de un estudiante
    pub async fn get_student_scholarships(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Scholarship>, sqlx::Error> {
        sqlx::query_as::<_, Scholarship>(
            r#"
            SELECT * FROM scholarships
            WHERE student_id = $1 AND is_active = TRUE
            ORDER BY created_at DESC
            "#
        )
        .bind(student_id)
        .fetch_all(&self.pool)
        .await
    }

    // ============================================
    // Facturas
    // ============================================

    /// Crear factura
    pub async fn create_invoice(
        &self,
        school_id: Uuid,
        student_id: Uuid,
        invoice_number: &str,
        invoice_type: &str,
        subtotal: rust_decimal::Decimal,
        tax: rust_decimal::Decimal,
        total: rust_decimal::Decimal,
        notes: Option<&str>,
    ) -> Result<Invoice, sqlx::Error> {
        sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoices (
                school_id, student_id, invoice_number, invoice_type,
                subtotal, tax, total, notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(student_id)
        .bind(invoice_number)
        .bind(invoice_type)
        .bind(subtotal)
        .bind(tax)
        .bind(total)
        .bind(notes)
        .fetch_one(&self.pool)
        .await
    }

    /// Crear item de factura
    pub async fn create_invoice_item(
        &self,
        invoice_id: Uuid,
        description: &str,
        quantity: i32,
        unit_price: rust_decimal::Decimal,
        amount: rust_decimal::Decimal,
        payment_concept_id: Option<Uuid>,
    ) -> Result<InvoiceItem, sqlx::Error> {
        sqlx::query_as::<_, InvoiceItem>(
            r#"
            INSERT INTO invoice_items (
                invoice_id, description, quantity, unit_price, amount,
                payment_concept_id
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(invoice_id)
        .bind(description)
        .bind(quantity)
        .bind(unit_price)
        .bind(amount)
        .bind(payment_concept_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener facturas de un estudiante
    pub async fn get_student_invoices(
        &self,
        student_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Invoice>, sqlx::Error> {
        sqlx::query_as::<_, Invoice>(
            r#"
            SELECT * FROM invoices
            WHERE student_id = $1
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(student_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    // ============================================
    // Reportes Financieros
    // ============================================

    /// Obtener resumen financiero de estudiante
    pub async fn get_student_financial_summary(
        &self,
        student_id: Uuid,
    ) -> Result<Option<StudentFinancialSummary>, sqlx::Error> {
        sqlx::query_as::<_, StudentFinancialSummary>(
            r#"
            SELECT * FROM student_financial_summary
            WHERE student_id = $1
            "#
        )
        .bind(student_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Obtener ingresos mensuales
    pub async fn get_monthly_revenue(
        &self,
        school_id: Uuid,
        months: i32,
    ) -> Result<Vec<MonthlyRevenue>, sqlx::Error> {
        sqlx::query_as::<_, MonthlyRevenue>(
            r#"
            SELECT 
                TO_CHAR(DATE_TRUNC('month', processed_at), 'YYYY-MM') as month,
                SUM(amount) as revenue
            FROM payments
            WHERE school_id = $1
              AND status = 'completed'
              AND processed_at >= NOW() - INTERVAL '$2 months'
            GROUP BY DATE_TRUNC('month', processed_at)
            ORDER BY month DESC
            "#
        )
        .bind(school_id)
        .bind(months)
        .fetch_all(&self.pool)
        .await
    }

    /// Dashboard financiero
    pub async fn get_finance_dashboard(
        &self,
        school_id: Uuid,
    ) -> Result<FinanceDashboard, sqlx::Error> {
        // Obtener métricas principales
        let total_revenue = sqlx::query_scalar::<_, Option<rust_decimal::Decimal>>(
            r#"
            SELECT COALESCE(SUM(amount), 0)
            FROM payments
            WHERE school_id = $1 AND status = 'completed'
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await?;

        let pending_revenue = sqlx::query_scalar::<_, Option<rust_decimal::Decimal>>(
            r#"
            SELECT COALESCE(SUM(total - paid_amount), 0)
            FROM pensions
            WHERE school_id = $1 AND status IN ('pending', 'partial')
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await?;

        let overdue_revenue = sqlx::query_scalar::<_, Option<rust_decimal::Decimal>>(
            r#"
            SELECT COALESCE(SUM(total - paid_amount), 0)
            FROM pensions
            WHERE school_id = $1 AND status = 'overdue'
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await?;

        let total_students = sqlx::query_scalar::<_, Option<i64>>(
            r#"
            SELECT COUNT(DISTINCT student_id)
            FROM pensions
            WHERE school_id = $1
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await?;

        let students_with_debt = sqlx::query_scalar::<_, Option<i64>>(
            r#"
            SELECT COUNT(DISTINCT student_id)
            FROM pensions
            WHERE school_id = $1
              AND status IN ('pending', 'partial', 'overdue')
              AND total > paid_amount
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await?;

        // Calcular collection rate
        let collection_rate = if let Some(total) = total_revenue {
            let pending = pending_revenue.unwrap_or(rust_decimal::Decimal::ZERO);
            let total_available = total + pending;
            if total_available > rust_decimal::Decimal::ZERO {
                (total / total_available) * rust_decimal::Decimal::from(100)
            } else {
                rust_decimal::Decimal::ZERO
            }
        } else {
            rust_decimal::Decimal::ZERO
        };

        // Obtener ingresos por mes
        let revenue_by_month = self.get_monthly_revenue(school_id, 12).await?;

        Ok(FinanceDashboard {
            total_revenue: total_revenue.unwrap_or(rust_decimal::Decimal::ZERO),
            pending_revenue: pending_revenue.unwrap_or(rust_decimal::Decimal::ZERO),
            overdue_revenue: overdue_revenue.unwrap_or(rust_decimal::Decimal::ZERO),
            collection_rate,
            total_students: total_students.unwrap_or(0),
            students_with_debt: students_with_debt.unwrap_or(0),
            revenue_by_month,
        })
    }
}
