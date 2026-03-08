use sqlx::{Pool, Postgres, Row};
use crate::models::{User, School, Course, Teacher, Student, Permission};
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

    pub async fn get_user_with_role(&self, email: &str) -> Result<Option<(User, String)>, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT 
                u.id, u.school_id, u.role_id, u.name, u.email, u.password_hash, u.created_at, u.updated_at,
                r.name as role_name
            FROM users u
            JOIN roles r ON u.role_id = r.id
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
                Ok(Some((user, role_name)))
            },
            None => Ok(None)
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
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.get("name")).collect())
    }

    // --- Schools ---

    pub async fn get_school_by_subdomain(&self, subdomain: &str) -> Result<Option<School>, sqlx::Error> {
        sqlx::query_as::<_, School>("SELECT * FROM schools WHERE subdomain = $1")
            .bind(subdomain)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_school(&self, name: &str, subdomain: &str) -> Result<School, sqlx::Error> {
        sqlx::query_as::<_, School>(
            "INSERT INTO schools (name, subdomain) VALUES ($1, $2) RETURNING *"
        )
        .bind(name)
        .bind(subdomain)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_schools(&self) -> Result<Vec<School>, sqlx::Error> {
        sqlx::query_as::<_, School>("SELECT * FROM schools")
            .fetch_all(&self.pool)
            .await
    }

    // --- Users ---

    pub async fn create_user(&self, school_id: Uuid, role_id: i32, name: &str, email: &str, password_hash: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(role_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
    }

    // --- Academic Module: Courses ---

    pub async fn list_courses(&self, school_id: Uuid) -> Result<Vec<Course>, sqlx::Error> {
        sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE school_id = $1")
            .bind(school_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_course(&self, school_id: Uuid, teacher_id: Option<Uuid>, name: &str, description: Option<&str>, grade_level: Option<&str>) -> Result<Course, sqlx::Error> {
        sqlx::query_as::<_, Course>(
            r#"
            INSERT INTO courses (school_id, teacher_id, name, description, grade_level)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
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

    pub async fn create_teacher(&self, school_id: Uuid, name: &str, email: &str, password_hash: &str, bio: Option<&str>, specialty: Option<&str>) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Create User
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, (SELECT id FROM roles WHERE name = 'profesor'), $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Create Teacher record
        sqlx::query(
            "INSERT INTO teachers (user_id, bio, specialty) VALUES ($1, $2, $3)"
        )
        .bind(user.id)
        .bind(bio)
        .bind(specialty)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn create_student(&self, school_id: Uuid, name: &str, email: &str, password_hash: &str, enrollment_number: Option<&str>, parent_id: Option<Uuid>) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Create User
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (school_id, role_id, name, email, password_hash)
            VALUES ($1, (SELECT id FROM roles WHERE name = 'alumno'), $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Create Student record
        sqlx::query(
            "INSERT INTO students (user_id, enrollment_number, parent_id) VALUES ($1, $2, $3)"
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
            "#
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
            "#
        )
        .bind(school_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Enrollments ---

    pub async fn enroll_student(&self, student_id: Uuid, course_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO enrollments (student_id, course_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(student_id)
        .bind(course_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn unenroll_student(&self, student_id: Uuid, course_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM enrollments WHERE student_id = $1 AND course_id = $2"
        )
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
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Grades ---

    pub async fn add_grade(&self, student_id: Uuid, course_id: Uuid, name: &str, grade: rust_decimal::Decimal) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO grades (student_id, course_id, name, grade) VALUES ($1, $2, $3, $4)"
        )
        .bind(student_id)
        .bind(course_id)
        .bind(name)
        .bind(grade)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_course_grades(&self, course_id: Uuid) -> Result<Vec<crate::models::GradeWithUser>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::GradeWithUser>(
            r#"
            SELECT g.id, g.name, g.grade, u.name as student_name
            FROM grades g
            JOIN users u ON g.student_id = u.id
            WHERE g.course_id = $1
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- Academic Module: Periods & Attendance ---

    pub async fn get_active_period(&self, school_id: Uuid) -> Result<Option<crate::models::AcademicPeriod>, sqlx::Error> {
        sqlx::query_as::<_, crate::models::AcademicPeriod>(
            "SELECT * FROM academic_periods WHERE school_id = $1 AND is_active = true LIMIT 1"
        )
        .bind(school_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn record_attendance(&self, student_id: Uuid, course_id: Uuid, date: chrono::NaiveDate, status: &str, notes: Option<&str>) -> Result<(), sqlx::Error> {
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

    pub async fn get_student_report_card(&self, student_id: Uuid) -> Result<Vec<crate::models::ReportCardItem>, sqlx::Error> {
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
