use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

// ============================================
// Modelos del Sistema
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PlatformSetting {
    pub id: i32,
    pub setting_key: String,
    pub setting_value: String,
    pub setting_type: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct School {
    pub id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub country_id: Option<i32>,
    pub is_system_admin: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    // Campos adicionales de ubicación
    pub address: Option<String>,
    pub comuna: Option<String>,
    pub provincia: Option<String>,
    pub estado: Option<String>,
    pub ciudad: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono: Option<String>,
    pub email_contacto: Option<String>,
    pub sitio_web: Option<String>,
    pub rut: Option<String>,
    pub razon_social: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct LegalRepresentative {
    pub id: Uuid,
    pub school_id: Uuid,
    pub nombre_completo: String,
    pub rut: String,
    pub cargo: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub es_principal: bool,
    pub fecha_nombramiento: Option<chrono::NaiveDate>,
    pub poder_notarial: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub school_id: Option<Uuid>,
    pub role_id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Teacher {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub specialty: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Student {
    pub user_id: Uuid,
    pub enrollment_number: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Course {
    pub id: Uuid,
    pub school_id: Uuid,
    pub teacher_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub grade_level: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Enrollment {
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub enrolled_at: Option<DateTime<Utc>>,
    pub status: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Grade {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub name: String,
    pub grade: rust_decimal::Decimal,
    pub weight: Option<rust_decimal::Decimal>,
    pub period_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct GradeWithUser {
    pub id: Uuid,
    pub name: String,
    pub grade: rust_decimal::Decimal,
    pub student_name: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AcademicPeriod {
    pub id: Uuid,
    pub school_id: Uuid,
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Attendance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub date: chrono::NaiveDate,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ReportCardItem {
    pub course_name: String,
    pub average_grade: rust_decimal::Decimal,
    pub attendance_percentage: rust_decimal::Decimal,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct SaasLicense {
    pub id: Uuid,
    pub school_id: Uuid,
    pub plan_type: String,
    pub status: String,
    pub expiry_date: DateTime<Utc>,
    pub auto_renew: bool,
    pub card_last4: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaasDashboardStats {
    pub total_schools: i64,
    pub active_licenses: i64,
    pub expiring_licenses: i64,
}

/// Extended stats for the root/platform-owner dashboard
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootDashboardStats {
    pub total_schools: i64,
    pub total_users: i64,
    pub active_licenses: i64,
    pub trial_licenses: i64,
    pub expiring_licenses: i64,
    pub expired_licenses: i64,
    pub mrr: rust_decimal::Decimal,
    pub annual_forecast: rust_decimal::Decimal,
    pub revenue_by_plan: HashMap<String, rust_decimal::Decimal>,
}

/// A license enriched with the school name
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseWithSchool {
    pub id: Uuid,
    pub school_id: Uuid,
    pub school_name: String,
    pub plan_type: String,
    pub status: String,
    pub expiry_date: DateTime<Utc>,
    pub auto_renew: bool,
    pub card_last4: Option<String>,
}

/// A school enriched with user count and license status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolWithStats {
    pub id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub is_system_admin: bool,
    pub user_count: i64,
    pub license_status: Option<String>,
    pub license_plan: Option<String>,
    pub country_code: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}
