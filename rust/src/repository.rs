use crate::models::{Course, School, User};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // --- Authentication & RBAC ---

    pub async fn get_user_with_role(
        &self,
        email: &str,
    ) -> Result<Option<(User, String, bool)>, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT 
                u.id, u.school_id, u.role_id, u.name, u.email, u.password_hash, u.created_at, u.updated_at,
                r.name as role_name,
                s.is_system_admin
            FROM users u
            JOIN roles r ON u.role_id = r.id
            JOIN schools s ON u.school_id = s.id
            WHERE u.email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    school_id: row.get("school_id"),
                    role_id: row.get("role_id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                let role_name: String = row.get("role_name");
                let is_system_admin: bool = row.get("is_system_admin");
                Ok(Some((user, role_name, is_system_admin)))
            }
            None => Ok(None),
        }
    }

    pub async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT p.name
            FROM users u
            JOIN role_permissions rp ON u.role_id = rp.role_id
            JOIN permissions p ON rp.permission_id = p.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.get("name")).collect())
    }

    // --- Schools ---

    pub async fn get_school_by_id(&self, id: Uuid) -> Result<Option<School>, sqlx::Error> {
        sqlx::query_as::<_, School>("SELECT * FROM schools WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_school_by_subdomain(
        &self,
        subdomain: &str,
    ) -> Result<Option<School>, sqlx::Error> {
        sqlx::query_as::<_, School>("SELECT * FROM schools WHERE subdomain = $1")
            .bind(subdomain)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_school(
        &self,
        id: Uuid,
        name: &str,
        subdomain: &str,
        country_id: Option<i32>,
    ) -> Result<School, sqlx::Error> {
        sqlx::query_as::<_, School>(
            "UPDATE schools SET name = $1, subdomain = $2, country_id = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4 RETURNING *"
        )
        .bind(name)
        .bind(subdomain)
        .bind(country_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_school(
        &self,
        name: &str,
        subdomain: &str,
        country_id: Option<i32>,
        is_system_admin: bool,
    ) -> Result<School, sqlx::Error> {
        sqlx::query_as::<_, School>(
            "INSERT INTO schools (name, subdomain, country_id, is_system_admin) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(name)
        .bind(subdomain)
        .bind(country_id)
        .bind(is_system_admin)
        .fetch_one(&self.pool)
        .await
    }
    pub async fn update_school_branding(
        &self,
        id: Uuid,
        logo_url: Option<&str>,
        primary_color: Option<&str>,
        secondary_color: Option<&str>,
    ) -> Result<School, sqlx::Error> {
        sqlx::query_as::<_, School>(
            r#"
            UPDATE schools 
            SET logo_url = $2, primary_color = $3, secondary_color = $4, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(logo_url)
        .bind(primary_color)
        .bind(secondary_color)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_schools(&self) -> Result<Vec<School>, sqlx::Error> {
        sqlx::query_as::<_, School>("SELECT * FROM schools")
            .fetch_all(&self.pool)
            .await
    }

    // --- SaaS Enterprise Layer ---

    pub async fn get_saas_stats(&self) -> Result<crate::models::SaasDashboardStats, sqlx::Error> {
        let total_schools = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM schools")
            .fetch_one(&self.pool)
            .await?;

        let active_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE status = 'active'",
        )
        .fetch_one(&self.pool)
        .await?;

        let expiring_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE expiry_date < (CURRENT_TIMESTAMP + INTERVAL '30 days') AND status = 'active'"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(crate::models::SaasDashboardStats {
            total_schools,
            active_licenses,
            expiring_licenses,
        })
    }

    pub async fn list_expiring_licenses(
        &self,
    ) -> Result<Vec<crate::models::SaasLicense>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::SaasLicense>(
            "SELECT * FROM saas_licenses WHERE expiry_date < (CURRENT_TIMESTAMP + INTERVAL '30 days') ORDER BY expiry_date ASC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_countries(&self) -> Result<Vec<crate::models::Country>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::Country>("SELECT * FROM countries ORDER BY name ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_root_dashboard_stats(
        &self,
    ) -> Result<crate::models::RootDashboardStats, sqlx::Error> {
        let total_schools = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM schools WHERE is_system_admin = false",
        )
        .fetch_one(&self.pool)
        .await?;

        let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        let active_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE status = 'active'",
        )
        .fetch_one(&self.pool)
        .await?;

        let trial_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE status = 'trial'",
        )
        .fetch_one(&self.pool)
        .await?;

        let expiring_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE expiry_date < (CURRENT_TIMESTAMP + INTERVAL '30 days') AND status = 'active'"
        ).fetch_one(&self.pool).await?;

        let expired_licenses = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM saas_licenses WHERE status = 'expired'",
        )
        .fetch_one(&self.pool)
        .await?;

        // Financial Calculations
        let licenses = sqlx::query(
            "SELECT plan_type, status FROM saas_licenses WHERE status IN ('active', 'trial')",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut mrr = rust_decimal::Decimal::ZERO;
        let mut revenue_by_plan = HashMap::new();

        for row in licenses {
            let plan: String = row.get("plan_type");
            let status: String = row.get("status");

            let amount = if status == "trial" {
                rust_decimal::Decimal::ZERO
            } else {
                match plan.as_str() {
                    "enterprise" => rust_decimal::Decimal::from(249),
                    "premium" => rust_decimal::Decimal::from(99),
                    _ => rust_decimal::Decimal::from(49), // basic
                }
            };

            mrr += amount;
            *revenue_by_plan
                .entry(plan)
                .or_insert(rust_decimal::Decimal::ZERO) += amount;
        }

        let annual_forecast = mrr * rust_decimal::Decimal::from(12);

        Ok(crate::models::RootDashboardStats {
            total_schools,
            total_users,
            active_licenses,
            trial_licenses,
            expiring_licenses,
            expired_licenses,
            mrr,
            annual_forecast,
            revenue_by_plan,
        })
    }

    pub async fn list_all_licenses_with_school(
        &self,
    ) -> Result<Vec<crate::models::LicenseWithSchool>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT l.id, l.school_id, s.name as school_name, l.plan_type, l.status,
                   l.expiry_date, l.auto_renew, l.card_last4
            FROM saas_licenses l
            JOIN schools s ON l.school_id = s.id
            ORDER BY l.expiry_date ASC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| crate::models::LicenseWithSchool {
                id: r.get("id"),
                school_id: r.get("school_id"),
                school_name: r.get("school_name"),
                plan_type: r.get("plan_type"),
                status: r.get("status"),
                expiry_date: r.get("expiry_date"),
                auto_renew: r.get("auto_renew"),
                card_last4: r.get("card_last4"),
            })
            .collect())
    }

    pub async fn upsert_license(
        &self,
        school_id: Uuid,
        plan_type: &str,
        status: &str,
        expiry_date: DateTime<Utc>,
        auto_renew: bool,
    ) -> Result<crate::models::SaasLicense, sqlx::Error> {
        sqlx::query_as::<_, crate::models::SaasLicense>(
            r#"
            INSERT INTO saas_licenses (school_id, plan_type, status, expiry_date, auto_renew)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (school_id) DO UPDATE SET
                plan_type = EXCLUDED.plan_type,
                status = EXCLUDED.status,
                expiry_date = EXCLUDED.expiry_date,
                auto_renew = EXCLUDED.auto_renew,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#,
        )
        .bind(school_id)
        .bind(plan_type)
        .bind(status)
        .bind(expiry_date)
        .bind(auto_renew)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_schools_with_stats(
        &self,
    ) -> Result<Vec<crate::models::SchoolWithStats>, sqlx::Error> {
        let rows = sqlx::query(r#"
            SELECT
                s.id, s.name, s.subdomain, s.is_system_admin,
                COUNT(DISTINCT u.id) as user_count,
                MAX(l.status) as license_status,
                MAX(l.plan_type) as license_plan,
                c.code as country_code,
                s.logo_url, s.primary_color, s.secondary_color
            FROM schools s
            LEFT JOIN users u ON u.school_id = s.id
            LEFT JOIN saas_licenses l ON l.school_id = s.id AND l.status = 'active'
            LEFT JOIN countries c ON s.country_id = c.id
            GROUP BY s.id, s.name, s.subdomain, s.is_system_admin, c.code, s.logo_url, s.primary_color, s.secondary_color
            ORDER BY s.created_at DESC
        "#)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| crate::models::SchoolWithStats {
                id: r.get("id"),
                name: r.get("name"),
                subdomain: r.get("subdomain"),
                is_system_admin: r.get("is_system_admin"),
                user_count: r.get("user_count"),
                license_status: r.get("license_status"),
                license_plan: r.get("license_plan"),
                country_code: r.get("country_code"),
                logo_url: r.get("logo_url"),
                primary_color: r.get("primary_color"),
                secondary_color: r.get("secondary_color"),
            })
            .collect())
    }

    // --- Users ---

    pub async fn create_user(
        &self,
        school_id: Uuid,
        role_id: i32,
        name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(school_id)
        .bind(role_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn bulk_create_users(
        &self,
        school_id: Uuid,
        users: Vec<(String, String, String, i32)>,
    ) -> Result<i64, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let mut count = 0;

        for (name, email, password_hash, role_id) in users {
            // First, insert or update user to get the ID
            let user = sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (school_id, role_id, name, email, password_hash)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (email) DO UPDATE SET name = EXCLUDED.name
                RETURNING *
                "#,
            )
            .bind(school_id)
            .bind(role_id)
            .bind(name)
            .bind(email)
            .bind(password_hash)
            .fetch_one(&mut *tx)
            .await?;

            // Populate role-specific tables
            if role_id == 2 {
                // profesor
                sqlx::query("INSERT INTO teachers (user_id) VALUES ($1) ON CONFLICT DO NOTHING")
                    .bind(user.id)
                    .execute(&mut *tx)
                    .await?;
            } else if role_id == 3 {
                // alumno
                sqlx::query("INSERT INTO students (user_id) VALUES ($1) ON CONFLICT DO NOTHING")
                    .bind(user.id)
                    .execute(&mut *tx)
                    .await?;
            }

            count += 1;
        }

        tx.commit().await?;
        Ok(count)
    }

    // --- Academic Module: Courses ---

    pub async fn list_courses(&self, school_id: Uuid) -> Result<Vec<Course>, sqlx::Error> {
        sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE school_id = $1")
            .bind(school_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_course(
        &self,
        school_id: Uuid,
        teacher_id: Option<Uuid>,
        name: &str,
        description: Option<&str>,
        grade_level: Option<&str>,
    ) -> Result<Course, sqlx::Error> {
        sqlx::query_as::<_, Course>(
            r#"
            INSERT INTO courses (school_id, teacher_id, name, description, grade_level)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(school_id)
        .bind(teacher_id)
        .bind(name)
        .bind(description)
        .bind(grade_level)
        .fetch_one(&self.pool)
        .await
    }

    // --- Academic Module: Teachers & Students ---

    pub async fn create_teacher(
        &self,
        school_id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
        bio: Option<&str>,
        specialty: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Create User
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, (SELECT id FROM roles WHERE name = 'profesor'), $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(school_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Create Teacher record
        sqlx::query("INSERT INTO teachers (user_id, bio, specialty) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(bio)
            .bind(specialty)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn create_student(
        &self,
        school_id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
        enrollment_number: Option<&str>,
        parent_id: Option<Uuid>,
    ) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Create User
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, (SELECT id FROM roles WHERE name = 'alumno'), $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(school_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Create Student record
        sqlx::query(
            "INSERT INTO students (user_id, enrollment_number, parent_id) VALUES ($1, $2, $3)",
        )
        .bind(user.id)
        .bind(enrollment_number)
        .bind(parent_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn list_teachers(&self, school_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT u.* FROM users u
            JOIN roles r ON u.role_id = r.id
            WHERE u.school_id = $1 AND r.name = 'profesor'
            "#,
        )
        .bind(school_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn list_students(&self, school_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT u.* FROM users u
            JOIN roles r ON u.role_id = r.id
            WHERE u.school_id = $1 AND r.name = 'alumno'
            "#,
        )
        .bind(school_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Enrollments ---

    pub async fn enroll_student(
        &self,
        student_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO enrollments (student_id, course_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(student_id)
        .bind(course_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn unenroll_student(
        &self,
        student_id: Uuid,
        course_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM enrollments WHERE student_id = $1 AND course_id = $2")
            .bind(student_id)
            .bind(course_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_course_students(&self, course_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT u.*
            FROM users u
            JOIN enrollments e ON u.id = e.student_id
            WHERE e.course_id = $1
            "#,
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Grades ---

    pub async fn add_grade(
        &self,
        student_id: Uuid,
        course_id: Uuid,
        name: &str,
        grade: rust_decimal::Decimal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO grades (student_id, course_id, name, grade) VALUES ($1, $2, $3, $4)",
        )
        .bind(student_id)
        .bind(course_id)
        .bind(name)
        .bind(grade)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_course_grades(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<crate::models::GradeWithUser>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::GradeWithUser>(
            r#"
            SELECT g.id, g.name, g.grade, u.name as student_name
            FROM grades g
            JOIN users u ON g.student_id = u.id
            WHERE g.course_id = $1
            "#,
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Periods & Attendance ---

    pub async fn get_active_period(
        &self,
        school_id: Uuid,
    ) -> Result<Option<crate::models::AcademicPeriod>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::AcademicPeriod>(
            "SELECT * FROM academic_periods WHERE school_id = $1 AND is_active = true LIMIT 1",
        )
        .bind(school_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn record_attendance(
        &self,
        student_id: Uuid,
        course_id: Uuid,
        date: chrono::NaiveDate,
        status: &str,
        notes: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO attendance (student_id, course_id, date, status, notes)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (student_id, course_id, date) DO UPDATE SET status = EXCLUDED.status, notes = EXCLUDED.notes
            "#
        )
        .bind(student_id)
        .bind(course_id)
        .bind(date)
        .bind(status)
        .bind(notes)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_student_report_card(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<crate::models::ReportCardItem>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::ReportCardItem>(
            r#"
            SELECT 
                c.name as course_name,
                COALESCE(AVG(g.grade), 0) as average_grade,
                (COUNT(CASE WHEN a.status = 'present' THEN 1 END)::NUMERIC / NULLIF(COUNT(a.id), 0)::NUMERIC) * 100 as attendance_percentage
            FROM enrollments e
            JOIN courses c ON e.course_id = c.id
            LEFT JOIN grades g ON e.student_id = g.student_id AND e.course_id = g.course_id
            LEFT JOIN attendance a ON e.student_id = a.student_id AND e.course_id = a.course_id
            WHERE e.student_id = $1
            GROUP BY c.id, c.name
            "#
        )
        .bind(student_id)
        .fetch_all(&self.pool)
        .await
    }
}
