use actix_web::{web, HttpResponse, get, post};
use sqlx::{Pool, Postgres};
use crate::repository::Repository;
use crate::HealthResponse;
use crate::auth::{verify_password, create_jwt, Claims};
use crate::models::{User, Course};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use tracing::debug;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub school_id: Uuid,
    pub role_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub name: String,
    pub description: Option<String>,
    pub teacher_id: Option<Uuid>,
    pub grade_level: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTeacherRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub specialty: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateStudentRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub enrollment_number: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[post("/auth/login")]
pub async fn login(repo: web::Data<Repository>, body: web::Json<LoginRequest>) -> HttpResponse {
    let user_role = repo.get_user_with_role(&body.email).await;

    match user_role {
        Ok(Some((user, role_name))) => {
            if verify_password(&body.password, &user.password_hash) {
                let school_id = user.school_id.unwrap_or_default();
                
                // Fetch permissions
                let permissions = repo.get_user_permissions(user.id).await.unwrap_or_default();
                
                match create_jwt(user.id, school_id, &role_name, permissions) {
                    Ok(token) => HttpResponse::Ok().json(json!({
                        "token": token,
                        "user": {
                            "id": user.id,
                            "name": user.name,
                            "email": user.email,
                            "role": role_name
                        }
                    })),
                    Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to create token"})),
                }
            } else {
                debug!("Password verification failed for user: {}", body.email);
                HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}))
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/auth/register")]
pub async fn register(repo: web::Data<Repository>, body: web::Json<RegisterRequest>) -> HttpResponse {
    use crate::auth::hash_password;
    
    let password_hash = hash_password(&body.password);
    
    let user_result: Result<User, sqlx::Error> = repo.create_user(
        body.school_id,
        body.role_id,
        &body.name,
        &body.email,
        &password_hash
    ).await;

    match user_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/auth/me")]
pub async fn get_me(claims: Claims) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "user_id": claims.sub,
        "school_id": claims.school_id,
        "role": claims.role,
        "permissions": claims.permissions
    }))
}

// --- Academic Module Handlers ---

#[get("/academic/courses")]
pub async fn list_courses(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.list_courses(school_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/academic/courses")]
pub async fn create_course(
    repo: web::Data<Repository>, 
    claims: Claims, 
    body: web::Json<CreateCourseRequest>
) -> HttpResponse {
    // RBAC: Solo admin o profesor pueden crear cursos
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.create_course(
        school_id,
        body.teacher_id,
        &body.name,
        body.description.as_deref(),
        body.grade_level.as_deref()
    ).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/teachers")]
pub async fn list_teachers(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.list_teachers(school_id).await {
        Ok(teachers) => HttpResponse::Ok().json(teachers),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/students")]
pub async fn list_students(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.list_students(school_id).await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/academic/teachers")]
pub async fn create_teacher(
    repo: web::Data<Repository>, 
    claims: Claims, 
    body: web::Json<CreateTeacherRequest>
) -> HttpResponse {
    // RBAC: Solo admin puede crear profesores
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    use crate::auth::hash_password;
    let password_hash = hash_password(&body.password);
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    match repo.create_teacher(
        school_id,
        &body.name,
        &body.email,
        &password_hash,
        body.bio.as_deref(),
        body.specialty.as_deref()
    ).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/academic/students")]
pub async fn create_student(
    repo: web::Data<Repository>, 
    claims: Claims, 
    body: web::Json<CreateStudentRequest>
) -> HttpResponse {
    // RBAC: Solo admin o profesor pueden registrar alumnos (depende de política)
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    use crate::auth::hash_password;
    let password_hash = hash_password(&body.password);
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    match repo.create_student(
        school_id,
        &body.name,
        &body.email,
        &password_hash,
        body.enrollment_number.as_deref(),
        body.parent_id
    ).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[derive(Deserialize)]
pub struct CreateEnrollmentRequest {
    pub student_id: Uuid,
    pub course_id: Uuid,
}

#[post("/academic/enrollments")]
pub async fn enroll_student(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<CreateEnrollmentRequest>
) -> HttpResponse {
    // RBAC: Admin o Profesor pueden matricular alumnos
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.enroll_student(body.student_id, body.course_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/courses/{id}/students")]
pub async fn list_course_students(
    repo: web::Data<Repository>,
    _claims: Claims,
    path: web::Path<Uuid>
) -> HttpResponse {
    let course_id = path.into_inner();
    match repo.list_course_students(course_id).await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[derive(Deserialize)]
pub struct CreateGradeRequest {
    pub student_id: Uuid,
    pub name: String,
    pub grade: rust_decimal::Decimal,
}

#[post("/academic/courses/{id}/grades")]
pub async fn add_grade(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<CreateGradeRequest>
) -> HttpResponse {
    let course_id = path.into_inner();
    
    // RBAC: Solo Admin o Profesor pueden poner notas
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.add_grade(body.student_id, course_id, &body.name, body.grade).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/courses/{id}/grades")]
pub async fn list_course_grades(
    repo: web::Data<Repository>,
    _claims: Claims,
    path: web::Path<Uuid>
) -> HttpResponse {
    let course_id = path.into_inner();
    match repo.list_course_grades(course_id).await {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// --- Academic Module: Attendance & Report Cards ---

#[derive(Deserialize)]
pub struct RecordAttendanceRequest {
    pub student_id: Uuid,
    pub date: chrono::NaiveDate,
    pub status: String,
    pub notes: Option<String>,
}

#[post("/academic/courses/{id}/attendance")]
pub async fn record_attendance(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<RecordAttendanceRequest>
) -> HttpResponse {
    let course_id = path.into_inner();
    
    // RBAC: Solo Admin o Profesor pueden pasar lista
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.record_attendance(body.student_id, course_id, body.date, &body.status, body.notes.as_deref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/my-report-card")]
pub async fn get_my_report_card(
    repo: web::Data<Repository>,
    claims: Claims
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    match repo.get_student_report_card(user_id).await {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/active-period")]
pub async fn get_active_period(
    repo: web::Data<Repository>,
    claims: Claims
) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.get_active_period(school_id).await {
        Ok(period) => HttpResponse::Ok().json(period),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// --- System Handlers ---

#[get("/health")]
pub async fn health(db_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let db_status = sqlx::query("SELECT 1")
        .execute(db_pool.get_ref())
        .await;

    let (message, db_connected) = match db_status {
        Ok(_) => ("Server is running and DB is connected".to_string(), true),
        Err(e) => (format!("Server is running but DB error: {}", e), false),
    };

    HttpResponse::Ok().json(HealthResponse {
        status: if db_connected { "ok".to_string() } else { "warning".to_string() },
        message,
        db_connected: Some(db_connected),
    })
}

#[get("/")]
pub async fn index(repo: web::Data<Repository>) -> HttpResponse {
    let schools = repo.get_all_schools().await.unwrap_or_default();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Colegio Backend API v1.0",
        "schools_found": schools.len(),
        "schools": schools
    }))
}
