use crate::HealthResponse;
use crate::auth::{Claims, create_jwt, hash_password, verify_password};
use crate::models::User;
use crate::repository::Repository;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, put, web};
use futures::TryStreamExt;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::io::Cursor;
use tracing::{debug, error};
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct UpdateSchoolRequest {
    pub name: String,
    pub subdomain: String,
    pub country_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateBrandingRequest {
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

#[derive(Deserialize)]
pub struct UpsertLicenseRequest {
    pub school_id: Uuid,
    pub plan_type: String,
    pub status: String,
    pub expiry_date: chrono::DateTime<chrono::Utc>,
    pub auto_renew: bool,
}

#[post("/auth/login")]
pub async fn login(repo: web::Data<Repository>, body: web::Json<LoginRequest>) -> HttpResponse {
    let user_role = repo.get_user_with_role(&body.email).await;

    match user_role {
        Ok(Some((user, role_name, is_system_admin))) => {
            if verify_password(&body.password, &user.password_hash) {
                let school_id = user.school_id.unwrap_or_default();

                // Fetch permissions
                let permissions = repo.get_user_permissions(user.id).await.unwrap_or_default();

                // Fetch school branding
                let school = repo.get_school_by_id(school_id).await.ok().flatten();

                match create_jwt(user.id, school_id, is_system_admin, &role_name, permissions) {
                    Ok(token) => HttpResponse::Ok().json(json!({
                        "token": token,
                        "user": {
                            "id": user.id,
                            "name": user.name,
                            "email": user.email,
                            "role": role_name,
                            "is_system_admin": is_system_admin
                        },
                        "school": school
                    })),
                    Err(_) => HttpResponse::InternalServerError()
                        .json(json!({"error": "Failed to create token"})),
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
pub async fn register(
    repo: web::Data<Repository>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    use crate::auth::hash_password;

    let password_hash = hash_password(&body.password);

    let user_result: Result<User, sqlx::Error> = repo
        .create_user(
            body.school_id,
            body.role_id,
            &body.name,
            &body.email,
            &password_hash,
        )
        .await;

    match user_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/auth/me")]
pub async fn get_me(_repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "id": claims.sub,
        "school_id": claims.school_id,
        "is_system_admin": claims.is_system_admin,
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
    body: web::Json<CreateCourseRequest>,
) -> HttpResponse {
    // RBAC: Solo admin o profesor pueden crear cursos
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo
        .create_course(
            school_id,
            body.teacher_id,
            &body.name,
            body.description.as_deref(),
            body.grade_level.as_deref(),
        )
        .await
    {
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
    body: web::Json<CreateTeacherRequest>,
) -> HttpResponse {
    // RBAC: Solo admin puede crear profesores
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    use crate::auth::hash_password;
    let password_hash = hash_password(&body.password);
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    match repo
        .create_teacher(
            school_id,
            &body.name,
            &body.email,
            &password_hash,
            body.bio.as_deref(),
            body.specialty.as_deref(),
        )
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/academic/students")]
pub async fn create_student(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<CreateStudentRequest>,
) -> HttpResponse {
    // RBAC: Solo admin o profesor pueden registrar alumnos (depende de política)
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    use crate::auth::hash_password;
    let password_hash = hash_password(&body.password);
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    match repo
        .create_student(
            school_id,
            &body.name,
            &body.email,
            &password_hash,
            body.enrollment_number.as_deref(),
            body.parent_id,
        )
        .await
    {
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
    body: web::Json<CreateEnrollmentRequest>,
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
    path: web::Path<Uuid>,
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
    body: web::Json<CreateGradeRequest>,
) -> HttpResponse {
    let course_id = path.into_inner();

    // RBAC: Solo Admin o Profesor pueden poner notas
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo
        .add_grade(body.student_id, course_id, &body.name, body.grade)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/courses/{id}/grades")]
pub async fn list_course_grades(
    repo: web::Data<Repository>,
    _claims: Claims,
    path: web::Path<Uuid>,
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
    body: web::Json<RecordAttendanceRequest>,
) -> HttpResponse {
    let course_id = path.into_inner();

    // RBAC: Solo Admin o Profesor pueden pasar lista
    if claims.role != "admin" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo
        .record_attendance(
            body.student_id,
            course_id,
            body.date,
            &body.status,
            body.notes.as_deref(),
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/my-report-card")]
pub async fn get_my_report_card(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    match repo.get_student_report_card(user_id).await {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/academic/active-period")]
pub async fn get_active_period(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    match repo.get_active_period(school_id).await {
        Ok(period) => HttpResponse::Ok().json(period),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// --- SaaS Enterprise Layer Handlers ---

#[get("/saas/stats")]
pub async fn get_saas_stats(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    // RBAC: Solo SuperAdmin (o admin con permiso saas:view_dashboard)
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:view_dashboard".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.get_saas_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/saas/licenses/expiring")]
pub async fn list_expiring_licenses(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:manage_licenses".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.list_expiring_licenses().await {
        Ok(licenses) => HttpResponse::Ok().json(licenses),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/saas/schools")]
pub async fn list_managed_schools(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:manage_schools".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.get_all_schools().await {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/saas/countries")]
pub async fn list_countries(repo: web::Data<Repository>, _claims: Claims) -> HttpResponse {
    match repo.list_countries().await {
        Ok(countries) => HttpResponse::Ok().json(countries),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/saas/schools")]
pub async fn create_managed_school(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:manage_schools".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let name = body["name"].as_str().unwrap_or_default();
    let subdomain = body["subdomain"].as_str().unwrap_or_default();
    let country_id = body["country_id"].as_i64().map(|id| id as i32);

    match repo.create_school(name, subdomain, country_id, false).await {
        Ok(school) => HttpResponse::Ok().json(school),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Root dashboard: enriched platform-wide stats (total users, licenses, schools, etc.)
#[get("/saas/dashboard")]
pub async fn get_root_dashboard(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }
    match repo.get_root_dashboard_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Full license listing with school name for the root console
#[get("/saas/licenses")]
pub async fn list_all_licenses(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }
    match repo.list_all_licenses_with_school().await {
        Ok(licenses) => HttpResponse::Ok().json(licenses),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/saas/licenses")]
pub async fn std_upsert_license(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<UpsertLicenseRequest>,
) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }

    match repo
        .upsert_license(
            body.school_id,
            &body.plan_type,
            &body.status,
            body.expiry_date,
            body.auto_renew,
        )
        .await
    {
        Ok(license) => HttpResponse::Ok().json(license),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Schools enriched with user counts and license status for the root console
#[get("/saas/schools/stats")]
pub async fn list_schools_stats(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }
    match repo.list_schools_with_stats().await {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Get a single school's details for the root console
#[get("/saas/schools/{id}")]
pub async fn get_school(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }
    let school_id = path.into_inner();
    match repo.get_school_by_id(school_id).await {
        Ok(Some(school)) => HttpResponse::Ok().json(school),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "School not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Update school information from the root console
#[put("/saas/schools/{id}")]
pub async fn update_school(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<UpdateSchoolRequest>,
) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }
    let school_id = path.into_inner();
    match repo
        .update_school(school_id, &body.name, &body.subdomain, body.country_id)
        .await
    {
        Ok(school) => HttpResponse::Ok().json(school),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// --- System Handlers ---

#[get("/health")]
pub async fn health(db_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let db_status = sqlx::query("SELECT 1").execute(db_pool.get_ref()).await;

    let (message, db_connected) = match db_status {
        Ok(_) => ("Server is running and DB is connected".to_string(), true),
        Err(e) => (format!("Server is running but DB error: {}", e), false),
    };

    HttpResponse::Ok().json(HealthResponse {
        status: if db_connected {
            "ok".to_string()
        } else {
            "warning".to_string()
        },
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
#[post("/admin/bulk-import")]
pub async fn bulk_import(
    repo: web::Data<Repository>,
    claims: Claims,
    mut payload: Multipart,
) -> HttpResponse {
    // RBAC check: Solo admin de la institución
    if claims.role != "admin" {
        return HttpResponse::Forbidden()
            .json(json!({"error": "Only admins can perform bulk imports"}));
    }

    let school_id = match Uuid::parse_str(&claims.school_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "Invalid school ID"})),
    };

    let mut users_to_create = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut bytes = Vec::new();
        while let Ok(Some(chunk)) = field.try_next().await {
            bytes.extend_from_slice(&chunk);
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(bytes));

        for result in rdr.deserialize::<BulkUserRecord>() {
            match result {
                Ok(record) => {
                    let role_id = match record.role.to_lowercase().as_str() {
                        "profesor" => 2,
                        "alumno" | "estudiante" => 3,
                        _ => continue,
                    };

                    let hashed = hash_password(&record.password);
                    users_to_create.push((record.name, record.email, hashed, role_id));
                }
                Err(e) => {
                    error!("CSV Deserialization error: {}", e);
                }
            }
        }
    }

    if users_to_create.is_empty() {
        return HttpResponse::BadRequest()
            .json(json!({"error": "No valid user data found in CSV"}));
    }

    match repo.bulk_create_users(school_id, users_to_create).await {
        Ok(count) => HttpResponse::Ok().json(json!({
            "message": "Bulk import completed successfully",
            "imported_count": count
        })),
        Err(e) => {
            error!("Bulk SQL error: {}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Database error during bulk import"}))
        }
    }
}

#[derive(Deserialize)]
struct BulkUserRecord {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[put("/admin/branding")]
pub async fn update_branding(
    repo: web::Data<Repository>,
    claims: crate::auth::Claims,
    body: web::Json<UpdateBrandingRequest>,
) -> HttpResponse {
    // Only admins can update branding
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(json!({"error": "Only admins can update branding"}));
    }

    let school_id = match Uuid::parse_str(&claims.school_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "Invalid school ID"})),
    };

    match repo
        .update_school_branding(
            school_id,
            body.logo_url.as_deref(),
            body.primary_color.as_deref(),
            body.secondary_color.as_deref(),
        )
        .await
    {
        Ok(school) => HttpResponse::Ok().json(school),
        Err(e) => {
            error!("Error updating branding: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Database error"}))
        }
    }
}
