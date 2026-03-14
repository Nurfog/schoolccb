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

// ============================================
// Modelos de Comunicaciones (Fase 6)
// ============================================

/// Tipo de notificación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    Academic,
    Financial,
}

/// Categoría de comunicado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementCategory {
    Urgent,
    Informative,
    Academic,
    Administrative,
}

/// Categoría de plantilla
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    Academic,
    Financial,
    Administrative,
    Marketing,
    General,
}

/// Canal de notificación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    InApp,
}

/// Estado de justificación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JustificationStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

/// Tipo de ausencia
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbsenceType {
    FullDay,
    Partial,
    Late,
    EarlyDeparture,
}

// ============================================
// Notificaciones
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NotificationWithSchool {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub school_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UnreadNotificationCount {
    pub user_id: Uuid,
    pub count: i64,
}

// ============================================
// Preferencias de Notificación
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub in_app_enabled: bool,
    pub categories: serde_json::Value,
    pub quiet_hours_enabled: bool,
    pub quiet_hours_start: Option<chrono::NaiveTime>,
    pub quiet_hours_end: Option<chrono::NaiveTime>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// Plantillas de Notificación
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub school_id: Option<Uuid>,
    pub name: String,
    pub code: String,
    pub subject: String,
    pub body: String,
    pub variables: serde_json::Value,
    pub category: String,
    pub channel: String,
    pub is_active: bool,
    pub is_system: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// Comunicados Escolares
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Announcement {
    pub id: Uuid,
    pub school_id: Uuid,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub target_audience: serde_json::Value,
    pub priority: i32,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allow_comments: bool,
    pub requires_confirmation: bool,
    pub attachment_urls: serde_json::Value,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AnnouncementWithAuthor {
    pub id: Uuid,
    pub school_id: Uuid,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub target_audience: serde_json::Value,
    pub priority: i32,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allow_comments: bool,
    pub requires_confirmation: bool,
    pub attachment_urls: serde_json::Value,
    pub created_by: Uuid,
    pub author_name: String,
    pub created_at: DateTime<Utc>,
}

// ============================================
// Lecturas de Comunicados
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AnnouncementReading {
    pub id: Uuid,
    pub announcement_id: Uuid,
    pub user_id: Uuid,
    pub is_confirmed: bool,
    pub read_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AnnouncementStats {
    pub announcement_id: Uuid,
    pub title: String,
    pub school_id: Uuid,
    pub category: String,
    pub is_published: bool,
    pub requires_confirmation: bool,
    pub total_read: i64,
    pub total_confirmed: i64,
    pub confirmation_percentage: Option<rust_decimal::Decimal>,
}

// ============================================
// Justificaciones de Inasistencia
// ============================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AttendanceJustification {
    pub id: Uuid,
    pub student_id: Uuid,
    pub parent_id: Uuid,
    pub school_id: Uuid,
    pub absence_date: chrono::NaiveDate,
    pub absence_type: String,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub reason: String,
    pub attachment_urls: serde_json::Value,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub review_notes: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AttendanceJustificationWithDetails {
    pub id: Uuid,
    pub student_id: Uuid,
    pub student_name: String,
    pub parent_id: Uuid,
    pub parent_name: String,
    pub school_id: Uuid,
    pub absence_date: chrono::NaiveDate,
    pub absence_type: String,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub reason: String,
    pub attachment_urls: serde_json::Value,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewer_name: Option<String>,
    pub review_notes: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// Request/Response Models para Comunicaciones
// ============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationCountResponse {
    pub unread_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementStatsResponse {
    pub announcement_id: Uuid,
    pub title: String,
    pub total_read: i64,
    pub total_confirmed: i64,
    pub confirmation_percentage: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnnouncementRequest {
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub target_audience: serde_json::Value,
    pub priority: Option<i32>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allow_comments: Option<bool>,
    pub requires_confirmation: Option<bool>,
    pub attachment_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAnnouncementRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub category: Option<String>,
    pub target_audience: Option<serde_json::Value>,
    pub priority: Option<i32>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allow_comments: Option<bool>,
    pub requires_confirmation: Option<bool>,
    pub attachment_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJustificationRequest {
    pub student_id: Uuid,
    pub absence_date: chrono::NaiveDate,
    pub absence_type: Option<String>,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewJustificationRequest {
    pub status: String,
    pub review_notes: Option<String>,
}

// ============================================
// Modelos de Seguridad y Auditoría (Fase 7)
// ============================================

/// Tipo de acción de auditoría
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    Login,
    Logout,
    PasswordChange,
    TwoFactorEnable,
    TwoFactorDisable,
    SessionRevoke,
    PermissionChange,
}

/// Log de auditoría
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub entity: String,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_method: Option<String>,
    pub request_path: Option<String>,
    pub status_code: Option<i32>,
    pub duration_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Sesión de usuario
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub refresh_token: Option<String>,
    pub device_info: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub is_current: bool,
    pub last_activity_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoke_reason: Option<String>,
}

/// Sesión con información del usuario
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserSessionWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub session_token: String,
    pub device_info: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub is_current: bool,
    pub last_activity_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Secreto 2FA
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User2faSecret {
    pub id: Uuid,
    pub user_id: Uuid,
    pub secret_key: String,
    pub backup_codes: Option<serde_json::Value>,
    pub is_enabled: bool,
    pub enabled_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Intento de login
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub email: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub user_id: Option<Uuid>,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Resumen de actividad de usuario
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserActivitySummary {
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub total_actions: i64,
    pub actions_last_24h: i64,
    pub total_logins: i64,
    pub logins_last_24h: i64,
    pub last_activity: Option<DateTime<Utc>>,
    pub unique_ips: i64,
}

/// Sesiones activas por usuario
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ActiveUserSessions {
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub active_sessions: i64,
    pub last_activity: Option<DateTime<Utc>>,
    pub earliest_expiry: Option<DateTime<Utc>>,
}

/// Intentos sospechosos de login
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct SuspiciousLoginAttempts {
    pub ip_address: Option<String>,
    pub email: String,
    pub failed_attempts: i64,
    pub first_attempt: DateTime<Utc>,
    pub last_attempt: DateTime<Utc>,
    pub user_agents: Vec<String>,
}

/// Estado de 2FA por usuario
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User2faStatus {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role_id: Option<i32>,
    pub has_2fa: bool,
    pub enabled_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
}

// ============================================
// Request/Response Models para Seguridad
// ============================================

/// Setup de 2FA
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
}

/// Verificación de 2FA
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorVerifyRequest {
    pub code: String,
}

/// Revocar sesión
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeSessionRequest {
    pub session_id: Uuid,
    pub reason: Option<String>,
}

/// Respuesta de sesiones activas
#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSessionsResponse {
    pub sessions: Vec<UserSessionWithUser>,
    pub total: i64,
}

/// Filtros para audit logs
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogFilters {
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub entity: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
}

// ============================================
// Modelos Financieros (Fase 8)
// ============================================

/// Categoría de concepto de pago
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentConceptCategory {
    Pension,
    Enrollment,
    Uniform,
    Books,
    Service,
    Other,
}

/// Estado de pensión
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PensionStatus {
    Pending,
    Paid,
    Partial,
    Overdue,
    Cancelled,
    Forgiven,
}

/// Método de pago
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    Card,
    Transfer,
    Stripe,
    Paypal,
    Check,
}

/// Estado de pago
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Completed,
    Pending,
    Cancelled,
    Refunded,
}

/// Tipo de factura
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    Receipt,
    Invoice,
    CreditNote,
}

/// Estado de factura
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Issued,
    Paid,
    Cancelled,
    Void,
}

// ============================================
// Entidades Financieras
// ============================================

/// Período financiero
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct FinancialPeriod {
    pub id: Uuid,
    pub school_id: Uuid,
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub is_active: bool,
    pub is_closed: bool,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

/// Concepto de pago
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PaymentConcept {
    pub id: Uuid,
    pub school_id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub category: String,
    pub amount: rust_decimal::Decimal,
    pub is_recurring: bool,
    pub frequency_months: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pensión mensual
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Pension {
    pub id: Uuid,
    pub school_id: Uuid,
    pub student_id: Uuid,
    pub financial_period_id: Option<Uuid>,
    pub payment_concept_id: Option<Uuid>,
    pub month: i32,
    pub year: i32,
    pub amount: rust_decimal::Decimal,
    pub discount: rust_decimal::Decimal,
    pub surcharge: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub due_date: chrono::NaiveDate,
    pub status: String,
    pub paid_amount: rust_decimal::Decimal,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pensión con datos del estudiante
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PensionWithStudent {
    pub id: Uuid,
    pub school_id: Uuid,
    pub student_id: Uuid,
    pub student_name: String,
    pub student_email: Option<String>,
    pub month: i32,
    pub year: i32,
    pub amount: rust_decimal::Decimal,
    pub discount: rust_decimal::Decimal,
    pub surcharge: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub due_date: chrono::NaiveDate,
    pub status: String,
    pub paid_amount: rust_decimal::Decimal,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Pago realizado
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub school_id: Uuid,
    pub student_id: Uuid,
    pub payer_id: Option<Uuid>,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub status: String,
    pub stripe_payment_intent_id: Option<String>,
    pub stripe_charge_id: Option<String>,
    pub processed_by: Option<Uuid>,
    pub processed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Aplicación de pago a pensión
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PaymentApplication {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub pension_id: Option<Uuid>,
    pub amount: rust_decimal::Decimal,
    pub created_at: DateTime<Utc>,
}

/// Beca/Descuento
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Scholarship {
    pub id: Uuid,
    pub school_id: Uuid,
    pub student_id: Uuid,
    pub name: String,
    pub type_field: String,
    pub value: rust_decimal::Decimal,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub is_active: bool,
    pub reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Factura/Recibo
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Invoice {
    pub id: Uuid,
    pub school_id: Uuid,
    pub student_id: Uuid,
    pub invoice_number: String,
    pub invoice_type: String,
    pub subtotal: rust_decimal::Decimal,
    pub tax: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub status: String,
    pub issued_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub stripe_invoice_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Item de factura
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub payment_concept_id: Option<Uuid>,
    pub description: String,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
    pub pension_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Recordatorio de pago
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PaymentReminder {
    pub id: Uuid,
    pub pension_id: Uuid,
    pub student_id: Uuid,
    pub reminder_type: String,
    pub status: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Resumen financiero de estudiante
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StudentFinancialSummary {
    pub student_id: Uuid,
    pub student_name: String,
    pub school_id: Uuid,
    pub school_name: String,
    pub total_pensions: i64,
    pub paid_pensions: i64,
    pub pending_pensions: i64,
    pub overdue_pensions: i64,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub paid_amount: Option<rust_decimal::Decimal>,
    pub outstanding_balance: Option<rust_decimal::Decimal>,
    pub overdue_amount: Option<rust_decimal::Decimal>,
}

// ============================================
// Request/Response Models Financieros
// ============================================

/// Crear pensión
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePensionRequest {
    pub student_id: Uuid,
    pub month: i32,
    pub year: i32,
    pub amount: rust_decimal::Decimal,
    pub discount: Option<rust_decimal::Decimal>,
    pub surcharge: Option<rust_decimal::Decimal>,
    pub due_date: chrono::NaiveDate,
}

/// Registrar pago
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub student_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub notes: Option<String>,
    pub pension_ids: Option<Vec<Uuid>>, // A qué pensiones se aplica
}

/// Crear factura
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub student_id: Uuid,
    pub invoice_type: String,
    pub items: Vec<InvoiceItemRequest>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemRequest {
    pub description: String,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub payment_concept_id: Option<Uuid>,
}

/// Crear beca
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScholarshipRequest {
    pub student_id: Uuid,
    pub name: String,
    pub type_field: String,
    pub value: rust_decimal::Decimal,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub reason: Option<String>,
}

/// Filtros para reportes financieros
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialReportFilters {
    pub school_id: Option<Uuid>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub student_id: Option<Uuid>,
}

/// Dashboard financiero
#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceDashboard {
    pub total_revenue: rust_decimal::Decimal,
    pub pending_revenue: rust_decimal::Decimal,
    pub overdue_revenue: rust_decimal::Decimal,
    pub collection_rate: rust_decimal::Decimal,
    pub total_students: i64,
    pub students_with_debt: i64,
    pub revenue_by_month: Vec<MonthlyRevenue>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct MonthlyRevenue {
    pub month: String,
    pub revenue: rust_decimal::Decimal,
}
