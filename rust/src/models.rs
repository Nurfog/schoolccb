use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct School {
    pub id: Uuid,
    pub name: String,
    pub subdomain: String,
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
