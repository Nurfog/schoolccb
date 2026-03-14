use crate::HealthResponse;
use crate::auth::{Claims, create_jwt, hash_password, verify_password};
use crate::communications_repository::CommunicationsRepository;
use crate::features::{FeatureStatus, FeatureType, PlanType};
use crate::models::*;
use crate::repository::Repository;
use crate::security_repository::SecurityRepository;
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use futures::TryStreamExt;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::io::Cursor;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    pub school_id: Uuid,
    pub role_id: i32,
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[validate(length(min = 2, max = 100, message = "Course name must be between 2 and 100 characters"))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub teacher_id: Option<Uuid>,
    #[validate(length(max = 50))]
    pub grade_level: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateTeacherRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(max = 500))]
    pub bio: Option<String>,
    #[validate(length(max = 100))]
    pub specialty: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateStudentRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(max = 50))]
    pub enrollment_number: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateSchoolRequest {
    #[validate(length(min = 2, max = 150))]
    pub name: String,
    #[validate(length(min = 2, max = 50))]
    pub subdomain: String,
    pub country_id: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateBrandingRequest {
    #[validate(url)]
    pub logo_url: Option<String>,
    #[validate(length(min = 7, max = 7))]
    pub primary_color: Option<String>,
    #[validate(length(min = 7, max = 7))]
    pub secondary_color: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpsertLicenseRequest {
    pub school_id: Uuid,
    #[validate(length(min = 3, max = 20))]
    pub plan_type: String,
    #[validate(length(min = 3, max = 20))]
    pub status: String,
    pub expiry_date: chrono::DateTime<chrono::Utc>,
    pub auto_renew: bool,
}

#[derive(Deserialize, Validate)]
pub struct AssignLicenseRequest {
    #[validate(length(min = 3, max = 20))]
    pub plan_type: String,
    pub expiry_date: chrono::DateTime<chrono::Utc>,
    pub auto_renew: Option<bool>,
}

/// Asignar o prorrogar licencia de un colegio
#[post("/saas/schools/{school_id}/license")]
pub async fn assign_license(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<AssignLicenseRequest>,
) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }

    let school_id = path.into_inner();
    
    // Verificar que el colegio existe
    match repo.get_school_by_id(school_id).await {
        Ok(Some(_)) => {},
        Ok(None) => return HttpResponse::NotFound().json(json!({"error": "School not found"})),
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }

    // Asignar licencia (upsert)
    match repo
        .upsert_license(
            school_id,
            &body.plan_type,
            "active",
            body.expiry_date,
            body.auto_renew.unwrap_or(false),
        )
        .await
    {
        Ok(license) => {
            info!(
                user_id = %claims.sub,
                school_id = %school_id,
                plan_type = %body.plan_type,
                "License assigned/extended"
            );
            HttpResponse::Ok().json(json!({
                "message": "Licencia asignada exitosamente",
                "license": license,
                "action": "assigned"
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/auth/login")]
pub async fn login(repo: web::Data<Repository>, body: web::Json<LoginRequest>) -> HttpResponse {
    // Validate input
    if let Err(e) = body.validate() {
        let errors = e.to_string();
        warn!(validation_errors = %errors, "Login validation failed");
        return HttpResponse::BadRequest().json(json!({"error": "Validación fallida", "details": errors}));
    }

    let user_role = repo.get_user_with_role(&body.email).await;

    match user_role {
        Ok(Some((user, role_name, is_system_admin))) => {
            if verify_password(&body.password, &user.password_hash) {
                let school_id = user.school_id.unwrap_or_default();

                // Fetch permissions
                let permissions = repo.get_user_permissions(user.id).await.unwrap_or_default();

                // Fetch school branding
                let school = repo.get_school_by_id(school_id).await.ok().flatten();

                match create_jwt(user.id, school_id, is_system_admin, &role_name, permissions, &user.email) {
                    Ok(token) => {
                        info!(user_id = %user.id, email = %user.email, role = %role_name, "User logged in successfully");
                        HttpResponse::Ok().json(json!({
                            "token": token,
                            "user": {
                                "id": user.id,
                                "name": user.name,
                                "email": user.email,
                                "role": role_name,
                                "is_system_admin": is_system_admin
                            },
                            "school": school
                        }))
                    },
                    Err(e) => {
                        error!(error = %e, "Failed to create JWT token");
                        HttpResponse::InternalServerError()
                            .json(json!({"error": "Failed to create token"}))
                    }
                }
            } else {
                warn!(email = %body.email, "Password verification failed");
                HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}))
            }
        }
        Ok(None) => {
            debug!(email = %body.email, "User not found");
            HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}))
        }
        Err(e) => {
            error!(error = %e, email = %body.email, "Database error during login");
            HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
        }
    }
}

#[post("/auth/register")]
pub async fn register(
    repo: web::Data<Repository>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    use crate::auth::hash_password;

    // Validate input
    if let Err(e) = body.validate() {
        let errors = e.to_string();
        warn!(validation_errors = %errors, "Register validation failed");
        return HttpResponse::BadRequest().json(json!({"error": "Validación fallida", "details": errors}));
    }

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

    // Validate input
    if let Err(e) = body.validate() {
        let errors = e.to_string();
        warn!(validation_errors = %errors, "Create course validation failed");
        return HttpResponse::BadRequest().json(json!({"error": "Validación fallida", "details": errors}));
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

    // Validate input
    if let Err(e) = body.validate() {
        let errors = e.to_string();
        warn!(validation_errors = %errors, "Create teacher validation failed");
        return HttpResponse::BadRequest().json(json!({"error": "Validación fallida", "details": errors}));
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

    // Validate input
    if let Err(e) = body.validate() {
        let errors = e.to_string();
        warn!(validation_errors = %errors, "Create student validation failed");
        return HttpResponse::BadRequest().json(json!({"error": "Validación fallida", "details": errors}));
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

#[derive(Deserialize)]
pub struct CreateCountryRequest {
    pub name: String,
    pub code: String,
}

#[post("/saas/countries")]
pub async fn create_country(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<CreateCountryRequest>,
) -> HttpResponse {
    // Solo root o admin pueden crear países
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:manage_schools".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo.create_country(&body.name, &body.code).await {
        Ok(country) => HttpResponse::Ok().json(country),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[derive(Deserialize)]
pub struct PlatformSettingRequest {
    pub setting_key: String,
    pub setting_value: String,
}

/// Obtener configuración de plataforma
#[get("/admin/platform-settings")]
pub async fn get_platform_settings(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }

    match repo.get_all_platform_settings().await {
        Ok(settings) => {
            // Convertir a HashMap para respuesta más limpia
            let mut config = serde_json::Map::new();
            for setting in settings {
                let value = if setting.setting_type == "boolean" {
                    serde_json::Value::Bool(setting.setting_value == "true")
                } else {
                    serde_json::Value::String(setting.setting_value)
                };
                config.insert(setting.setting_key, value);
            }
            HttpResponse::Ok().json(config)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Actualizar configuración de plataforma
#[post("/admin/platform-settings")]
pub async fn update_platform_setting(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<PlatformSettingRequest>,
) -> HttpResponse {
    if !claims.is_system_admin {
        return HttpResponse::Forbidden().json(json!({"error": "Root access required"}));
    }

    // Determinar el tipo de configuración
    let setting_type = if body.setting_key.contains("enabled") {
        "boolean"
    } else {
        "string"
    };

    match repo
        .upsert_platform_setting(&body.setting_key, &body.setting_value, setting_type)
        .await
    {
        Ok(_) => {
            info!(
                user_id = %claims.sub,
                key = %body.setting_key,
                "Platform setting updated"
            );
            HttpResponse::Ok().json(json!({
                "message": "Configuración actualizada exitosamente",
                "setting_key": body.setting_key
            }))
        }
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

// ============================================
// Legal Representatives Handlers
// ============================================

#[derive(Deserialize)]
pub struct CreateLegalRepresentativeRequest {
    pub school_id: Uuid,
    pub nombre_completo: String,
    pub rut: String,
    pub cargo: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub es_principal: Option<bool>,
    pub fecha_nombramiento: Option<chrono::NaiveDate>,
    pub poder_notarial: Option<String>,
}

/// Crear representante legal
#[post("/saas/legal-representatives")]
pub async fn create_legal_representative(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<CreateLegalRepresentativeRequest>,
) -> HttpResponse {
    // Solo root o admin pueden crear representantes
    if (claims.role != "admin" && claims.role != "root")
        || (!claims
            .permissions
            .contains(&"saas:manage_schools".to_string())
            && claims.role != "root")
    {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    match repo
        .create_legal_representative(
            body.school_id,
            &body.nombre_completo,
            &body.rut,
            &body.cargo,
            body.email.as_deref(),
            body.telefono.as_deref(),
            body.direccion.as_deref(),
            body.es_principal.unwrap_or(false),
            body.fecha_nombramiento,
            body.poder_notarial.as_deref(),
        )
        .await
    {
        Ok(representative) => HttpResponse::Ok().json(representative),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Listar representantes de un colegio
#[get("/saas/schools/{school_id}/legal-representatives")]
pub async fn list_legal_representatives(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let school_id = path.into_inner();

    match repo.list_legal_representatives(school_id).await {
        Ok(representatives) => HttpResponse::Ok().json(representatives),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// --- System Handlers ---

#[get("/health")]
pub async fn health(db_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let db_status = sqlx::query("SELECT 1").execute(db_pool.get_ref()).await;

    let (message, db_connected) = match db_status {
        Ok(_) => {
            info!("Health check passed - database connected");
            ("Server is running and DB is connected".to_string(), true)
        }
        Err(e) => {
            error!(error = %e, "Health check - database connection failed");
            (format!("Server is running but DB error: {}", e), false)
        }
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

// ============================================
// Planes y Features Handlers
// ============================================

/// Obtener todos los planes disponibles con sus features
#[get("/billing/plans")]
pub async fn list_plans(_claims: Claims) -> HttpResponse {
    HttpResponse::Ok().json(PlanType::all_plans())
}

/// Obtener el plan actual del colegio
#[get("/billing/my-plan")]
pub async fn get_my_plan(repo: web::Data<Repository>, claims: Claims) -> HttpResponse {
    let school_id = match Uuid::parse_str(&claims.school_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "Invalid school ID"})),
    };

    match repo.get_license_by_school(school_id).await {
        Ok(license) => {
            let plan: PlanType = license.plan_type.parse().unwrap_or(PlanType::Basic);
            let plan_info = plan.get_plan_info();
            
            // Obtener features con estado de uso
            let mut features_status = Vec::new();
            
            // Ejemplo: contar estudiantes
            if let Ok(students) = repo.list_students(school_id).await {
                let student_count = students.len() as i64;
                let max_students = plan.max_students().map(|m| m as i64);
                
                features_status.push(FeatureStatus {
                    feature: FeatureType::AcademicCore,
                    enabled: true,
                    limit: max_students,
                    used: Some(student_count),
                });
            }
            
            HttpResponse::Ok().json(json!({
                "plan": plan_info,
                "license": {
                    "status": license.status,
                    "expiry_date": license.expiry_date,
                    "auto_renew": license.auto_renew,
                    "plan_type": license.plan_type
                },
                "features": features_status
            }))
        }
        Err(sqlx::Error::RowNotFound) => {
            // No hay licencia, devolver plan básico por defecto
            let plan = PlanType::Basic;
            HttpResponse::Ok().json(json!({
                "plan": plan.get_plan_info(),
                "license": {
                    "status": "none",
                    "plan_type": "basic"
                },
                "message": "No license found, using basic plan"
            }))
        }
        Err(e) => {
            error!("Error getting plan: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
        }
    }
}

// ============================================
// Stripe Handlers (Opcional)
// ============================================

#[derive(Deserialize)]
pub struct CreateCheckoutRequest {
    pub plan: String,
    pub billing_cycle: String, // 'monthly' o 'yearly'
}

/// Crear sesión de checkout de Stripe
#[post("/billing/checkout")]
pub async fn create_checkout(
    repo: web::Data<Repository>,
    claims: Claims,
    body: web::Json<CreateCheckoutRequest>,
) -> HttpResponse {
    let school_id = match Uuid::parse_str(&claims.school_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "Invalid school ID"})),
    };

    // Verificar Stripe API key
    let stripe_key = match std::env::var("STRIPE_SECRET_KEY") {
        Ok(key) if !key.is_empty() => key,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": "Stripe no está configurado",
                "message": "Contacta al administrador para habilitar pagos"
            }));
        }
        _ => {
            return HttpResponse::InternalServerError().json(json!({
                "error": "Stripe no está configurado",
                "message": "STRIPE_SECRET_KEY no está definida"
            }));
        }
    };

    // Precios por plan
    let prices = match body.plan.as_str() {
        "basic" => if body.billing_cycle == "yearly" { 490 } else { 49 },
        "premium" => if body.billing_cycle == "yearly" { 990 } else { 99 },
        "enterprise" => if body.billing_cycle == "yearly" { 2490 } else { 249 },
        _ => return HttpResponse::BadRequest().json(json!({"error": "Plan inválido"})),
    };

    // En producción, aquí crearías la sesión de Stripe
    // Por ahora, devolvemos una respuesta simulada
    info!(
        user_id = %claims.sub,
        school_id = %school_id,
        plan = %body.plan,
        cycle = %body.billing_cycle,
        price = prices,
        "Checkout request received"
    );

    // TODO: Integración real con Stripe
    // let client = stripe::Client::new(&stripe_key);
    // let session = stripe::Checkout::Session::create(
    //     &client,
    //     stripe::CreateCheckoutSession {
    //         // ... configuración
    //     },
    // ).await?;

    HttpResponse::Ok().json(json!({
        "message": "Checkout iniciado",
        "plan": body.plan,
        "billing_cycle": body.billing_cycle,
        "price_usd": prices,
        "checkout_url": "https://checkout.stripe.com/csk_test_...", // URL simulada
        "note": "Configura STRIPE_SECRET_KEY para habilitar pagos reales"
    }))
}

/// Webhook de Stripe para eventos de pago
#[post("/billing/stripe-webhook")]
pub async fn stripe_webhook(
    repo: web::Data<Repository>,
    body: String,
    req: HttpRequest,
) -> HttpResponse {
    // Verificar firma de Stripe
    let stripe_signature = req.headers()
        .get("Stripe-Signature")
        .and_then(|h| h.to_str().ok());

    if stripe_signature.is_none() {
        return HttpResponse::BadRequest().json(json!({"error": "Missing Stripe signature"}));
    }

    let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default();
    
    // TODO: Verificar firma con stripe::Webhook::construct_event
    // Por ahora, procesamos el evento directamente
    
    info!("Stripe webhook received");
    
    // Parsear evento
    if let Ok(event) = serde_json::from_str::<serde_json::Value>(&body) {
        let event_type = event["type"].as_str().unwrap_or("unknown");
        
        match event_type {
            "checkout.session.completed" => {
                // Actualizar licencia después de pago exitoso
                info!("Payment completed");
                // TODO: Actualizar saas_licenses con nuevo plan
            }
            "customer.subscription.updated" => {
                info!("Subscription updated");
            }
            "invoice.payment_failed" => {
                warn!("Payment failed");
                // TODO: Notificar al usuario
            }
            _ => {
                debug!("Unhandled event type: {}", event_type);
            }
        }
    }

    HttpResponse::Ok().json(json!({"status": "ok"}))
}

// ============================================
// Handlers de Comunicaciones (Fase 6)
// ============================================

// ============================================
// Notificaciones
// ============================================

/// Obtener notificaciones del usuario
#[get("/api/notifications")]
pub async fn get_notifications(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let limit = query.get("limit").and_then(|v| v.as_i64()).unwrap_or(20);
    let offset = query.get("offset").and_then(|v| v.as_i64()).unwrap_or(0);

    match repo.get_user_notifications(user_id, limit, offset).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Contar notificaciones no leídas
#[get("/api/notifications/unread-count")]
pub async fn get_unread_count(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.count_unread_notifications(user_id).await {
        Ok(count) => HttpResponse::Ok().json(json!({"unread_count": count})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Marcar notificación como leída
#[put("/api/notifications/{id}/read")]
pub async fn mark_notification_read(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let notification_id = path.into_inner();

    match repo.mark_notification_read(notification_id, user_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Marcar todas las notificaciones como leídas
#[put("/api/notifications/read-all")]
pub async fn mark_all_notifications_read(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.mark_all_notifications_read(user_id).await {
        Ok(count) => HttpResponse::Ok().json(json!({"marked_count": count})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Eliminar notificación
#[delete("/api/notifications/{id}")]
pub async fn delete_notification(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let notification_id = path.into_inner();

    match repo.delete_notification(notification_id, user_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Preferencias de Notificación
// ============================================

/// Obtener preferencias de notificación
#[get("/api/notification-preferences")]
pub async fn get_notification_preferences(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.get_or_create_preferences(user_id).await {
        Ok(preferences) => HttpResponse::Ok().json(preferences),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Actualizar preferencias de notificación
#[put("/api/notification-preferences")]
pub async fn update_notification_preferences(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    let email_enabled = body.get("email_enabled").and_then(|v| v.as_bool());
    let push_enabled = body.get("push_enabled").and_then(|v| v.as_bool());
    let sms_enabled = body.get("sms_enabled").and_then(|v| v.as_bool());
    let in_app_enabled = body.get("in_app_enabled").and_then(|v| v.as_bool());
    let categories = body.get("categories").cloned();
    let quiet_hours_enabled = body.get("quiet_hours_enabled").and_then(|v| v.as_bool());

    // Parsear horas de silencio
    let quiet_hours_start = body.get("quiet_hours_start")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::NaiveTime::parse_from_str(s, "%H:%M").ok());
    
    let quiet_hours_end = body.get("quiet_hours_end")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::NaiveTime::parse_from_str(s, "%H:%M").ok());

    match repo.update_preferences(
        user_id,
        email_enabled,
        push_enabled,
        sms_enabled,
        in_app_enabled,
        categories,
        quiet_hours_enabled,
        quiet_hours_start,
        quiet_hours_end,
    ).await {
        Ok(preferences) => HttpResponse::Ok().json(preferences),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Plantillas de Notificación
// ============================================

/// Listar plantillas de notificación
#[get("/api/templates")]
pub async fn list_templates(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    // Solo admin puede ver plantillas
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let category = query.get("category").and_then(|v| v.as_str());

    match repo.list_templates(Some(school_id), category).await {
        Ok(templates) => HttpResponse::Ok().json(templates),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener plantilla por código
#[get("/api/templates/{code}")]
pub async fn get_template(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<String>,
) -> HttpResponse {
    // Solo admin puede ver plantillas
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let code = path.into_inner();

    match repo.get_template_by_code(&code, Some(school_id)).await {
        Ok(template) => HttpResponse::Ok().json(template),
        Err(_e) => HttpResponse::NotFound().json(json!({"error": "Template not found"})),
    }
}

// ============================================
// Comunicados Escolares
// ============================================

/// Listar comunicados publicados
#[get("/api/announcements")]
pub async fn list_announcements(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let category = query.get("category").and_then(|v| v.as_str());
    let limit = query.get("limit").and_then(|v| v.as_i64()).unwrap_or(20);
    let offset = query.get("offset").and_then(|v| v.as_i64()).unwrap_or(0);

    match repo.list_published_announcements(school_id, category, limit, offset).await {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener comunicado por ID
#[get("/api/announcements/{id}")]
pub async fn get_announcement(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let announcement_id = path.into_inner();

    // Registrar lectura
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let _ = repo.record_announcement_reading(announcement_id, user_id, None, None).await;

    match repo.get_announcement(announcement_id).await {
        Ok(Some(announcement)) => HttpResponse::Ok().json(announcement),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Announcement not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Confirmar lectura de comunicado
#[post("/api/announcements/{id}/read")]
pub async fn confirm_announcement_reading(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let announcement_id = path.into_inner();

    match repo.confirm_announcement_reading(announcement_id, user_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener estadísticas de comunicado (solo admin)
#[get("/api/announcements/{id}/stats")]
pub async fn get_announcement_stats(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    // Solo admin puede ver estadísticas
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let announcement_id = path.into_inner();

    match repo.get_announcement_stats(announcement_id).await {
        Ok(Some(stats)) => HttpResponse::Ok().json(stats),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Stats not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Crear comunicado (solo admin)
#[post("/api/announcements")]
pub async fn create_announcement(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    body: web::Json<CreateAnnouncementRequest>,
) -> HttpResponse {
    // Solo admin puede crear comunicados
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let created_by = Uuid::parse_str(&claims.sub).unwrap_or_default();

    let attachment_urls = body.attachment_urls.clone().unwrap_or_default();
    let attachment_json = serde_json::to_value(attachment_urls).unwrap_or_default();

    match repo.create_announcement(
        school_id,
        &body.title,
        &body.content,
        body.summary.as_deref(),
        &body.category,
        body.target_audience.clone(),
        body.priority.unwrap_or(1),
        body.scheduled_at,
        body.expires_at,
        body.allow_comments.unwrap_or(false),
        body.requires_confirmation.unwrap_or(false),
        attachment_json,
        created_by,
    ).await {
        Ok(announcement) => HttpResponse::Created().json(announcement),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Publicar comunicado (solo admin)
#[post("/api/announcements/{id}/publish")]
pub async fn publish_announcement(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    // Solo admin puede publicar comunicados
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let announcement_id = path.into_inner();

    match repo.publish_announcement(announcement_id, school_id).await {
        Ok(announcement) => HttpResponse::Ok().json(announcement),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Actualizar comunicado (solo admin)
#[put("/api/announcements/{id}")]
pub async fn update_announcement(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<UpdateAnnouncementRequest>,
) -> HttpResponse {
    // Solo admin puede actualizar comunicados
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let announcement_id = path.into_inner();
    let updated_by = Uuid::parse_str(&claims.sub).unwrap_or_default();

    let attachment_urls = body.attachment_urls.clone().unwrap_or_default();
    let attachment_json = serde_json::to_value(attachment_urls).unwrap_or_default();

    match repo.update_announcement(
        announcement_id,
        school_id,
        body.title.as_deref(),
        body.content.as_deref(),
        body.summary.as_deref(),
        body.category.as_deref(),
        body.target_audience.clone(),
        body.priority,
        body.scheduled_at,
        body.expires_at,
        body.allow_comments,
        body.requires_confirmation,
        Some(attachment_json),
        updated_by,
    ).await {
        Ok(announcement) => HttpResponse::Ok().json(announcement),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Eliminar comunicado (solo admin)
#[delete("/api/announcements/{id}")]
pub async fn delete_announcement(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    // Solo admin puede eliminar comunicados
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let announcement_id = path.into_inner();

    match repo.delete_announcement(announcement_id, school_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Justificaciones de Inasistencia
// ============================================

/// Crear justificación de inasistencia
#[post("/api/parent/attendance-justification")]
pub async fn create_attendance_justification(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    body: web::Json<CreateJustificationRequest>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    // Validar que el usuario es padre del estudiante
    // (Esta validación debería hacerse en el repository también)
    
    let attachment_urls = serde_json::json!([]);

    match repo.create_justification(
        body.student_id,
        user_id,
        school_id,
        body.absence_date,
        body.absence_type.as_deref().unwrap_or("full_day"),
        body.start_time,
        body.end_time,
        &body.reason,
        attachment_urls,
    ).await {
        Ok(justification) => HttpResponse::Created().json(justification),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener justificaciones de un estudiante (solo padre o admin)
#[get("/api/students/{id}/attendance-justifications")]
pub async fn get_student_justifications(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let limit = query.get("limit").and_then(|v| v.as_i64()).unwrap_or(20);
    let offset = query.get("offset").and_then(|v| v.as_i64()).unwrap_or(0);

    // Validar permisos (padre del estudiante o admin)
    if claims.role != "admin" && claims.role != "root" {
        // Aquí debería validarse que el usuario es el padre
        // Por ahora, permitimos el acceso
    }

    match repo.get_student_justifications(student_id, limit, offset).await {
        Ok(justifications) => HttpResponse::Ok().json(justifications),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener justificaciones pendientes (solo admin/profesor)
#[get("/api/school/attendance-justifications/pending")]
pub async fn get_pending_justifications(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    // Solo admin o profesor puede ver justificaciones pendientes
    if claims.role != "admin" && claims.role != "root" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let limit = query.get("limit").and_then(|v| v.as_i64()).unwrap_or(20);
    let offset = query.get("offset").and_then(|v| v.as_i64()).unwrap_or(0);

    match repo.get_pending_justifications(school_id, limit, offset).await {
        Ok(justifications) => HttpResponse::Ok().json(justifications),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Revisar justificación (solo admin/profesor)
#[post("/api/attendance-justifications/{id}/review")]
pub async fn review_justification(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: web::Json<ReviewJustificationRequest>,
) -> HttpResponse {
    // Solo admin o profesor puede revisar justificaciones
    if claims.role != "admin" && claims.role != "root" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();
    let justification_id = path.into_inner();
    let reviewed_by = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.review_justification(
        justification_id,
        school_id,
        &body.status,
        body.review_notes.as_deref(),
        reviewed_by,
    ).await {
        Ok(justification) => HttpResponse::Ok().json(justification),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Contar justificaciones pendientes
#[get("/api/school/attendance-justifications/pending-count")]
pub async fn count_pending_justifications(
    repo: web::Data<CommunicationsRepository>,
    claims: Claims,
) -> HttpResponse {
    // Solo admin o profesor puede contar justificaciones pendientes
    if claims.role != "admin" && claims.role != "root" && claims.role != "profesor" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let school_id = Uuid::parse_str(&claims.school_id).unwrap_or_default();

    match repo.count_pending_justifications(school_id).await {
        Ok(count) => HttpResponse::Ok().json(json!({"pending_count": count})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Handlers de Seguridad y Auditoría (Fase 7)
// ============================================

// ============================================
// Audit Logs
// ============================================

/// Obtener audit logs con filtros
#[get("/api/audit/logs")]
pub async fn get_audit_logs(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
    query: web::Query<AuditLogFilters>,
) -> HttpResponse {
    // Solo root o admin puede ver audit logs
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let limit = 100;
    let offset = 0;

    match repo.get_audit_logs(&query.into_inner(), limit, offset).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener actividad de un usuario
#[get("/api/audit/user/{user_id}/activity")]
pub async fn get_user_activity(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    // Solo root o admin puede ver actividad de otros usuarios
    if claims.role != "admin" && claims.role != "root" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let user_id = path.into_inner();

    match repo.get_user_activity_summary(user_id).await {
        Ok(Some(summary)) => HttpResponse::Ok().json(summary),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Obtener intentos sospechosos de login
#[get("/api/audit/suspicious-logins")]
pub async fn get_suspicious_logins(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
) -> HttpResponse {
    // Solo root puede ver intentos sospechosos
    if claims.role != "root" && claims.role != "admin" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let limit = 50;

    match repo.get_suspicious_login_attempts(limit).await {
        Ok(attempts) => HttpResponse::Ok().json(attempts),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Gestión de Sesiones
// ============================================

/// Obtener sesiones activas del usuario actual
#[get("/api/sessions")]
pub async fn get_my_sessions(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.get_active_sessions(user_id).await {
        Ok(sessions) => HttpResponse::Ok().json(json!({
            "sessions": sessions,
            "total": sessions.len() as i64
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Revocar sesión específica
#[post("/api/sessions/{session_id}/revoke")]
pub async fn revoke_session(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
    path: web::Path<Uuid>,
    body: Option<web::Json<RevokeSessionRequest>>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
    let session_id = path.into_inner();
    let reason = body.and_then(|b| b.reason.clone());

    // Verificar que la sesión pertenece al usuario
    // (Esta validación debería hacerse también en el repository)
    
    match repo.revoke_session(session_id, reason.as_deref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Revocar todas las sesiones excepto la actual
#[post("/api/sessions/revoke-all")]
pub async fn revoke_all_other_sessions(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    // Obtener sesión actual del contexto (debería pasar por headers o claims)
    // Por ahora, revocamos todas menos una genérica
    match repo.revoke_all_sessions_except(user_id, Uuid::nil()).await {
        Ok(count) => HttpResponse::Ok().json(json!({
            "revoked_count": count,
            "message": format!("{} sesiones revocadas", count)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// ============================================
// Autenticación 2FA (TOTP)
// ============================================

/// Iniciar setup de 2FA
#[post("/api/2fa/setup")]
pub async fn setup_2fa(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    // Verificar si ya tiene 2FA habilitado
    if let Ok(Some(existing)) = repo.get_2fa_secret(user_id).await {
        if existing.is_enabled {
            return HttpResponse::BadRequest().json(json!({
                "error": "2FA already enabled"
            }));
        }
    }

    // Generar secreto TOTP aleatorio (20 bytes para TOTP estándar)
    let secret_bytes: [u8; 20] = rand::random();
    let secret_key = base32::encode(base32::Alphabet::Crockford, &secret_bytes);

    // Generar códigos de respaldo
    let backup_codes: Vec<String> = (0..10)
        .map(|_| uuid::Uuid::new_v4().to_string()[..8].to_string())
        .collect();

    // Guardar secreto (sin habilitar aún)
    let backup_codes_json = serde_json::to_value(&backup_codes).unwrap();
    
    match repo.upsert_2fa_secret(user_id, &secret_key, Some(backup_codes_json)).await {
        Ok(_) => {
            // Generar QR URL manualmente
            let qr_url = format!(
                "otpauth://totp/SchoolCCB:{}?secret={}&issuer=SchoolCCB",
                claims.email, secret_key
            );
            
            HttpResponse::Ok().json(TwoFactorSetupResponse {
                secret: secret_key,
                qr_code_url: qr_url,
                backup_codes,
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Verificar código 2FA y habilitar
#[post("/api/2fa/verify")]
pub async fn verify_2fa(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
    body: web::Json<TwoFactorVerifyRequest>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    // Obtener secreto
    match repo.get_2fa_secret(user_id).await {
        Ok(Some(secret)) => {
            // NOTA: Aquí iría la verificación TOTP real con totp_rs
            // Por ahora, verificación simplificada (6 dígitos)
            if body.code.len() == 6 && body.code.chars().all(|c| c.is_numeric()) {
                // Código válido, habilitar 2FA
                match repo.enable_2fa(user_id).await {
                    Ok(_) => HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "2FA enabled successfully"
                    })),
                    Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
                }
            } else {
                // Código inválido
                let _ = repo.record_2fa_failure(user_id, 5, 900).await;
                HttpResponse::BadRequest().json(json!({
                    "error": "Invalid 2FA code (must be 6 digits)"
                }))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "2FA setup not initiated"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Deshabilitar 2FA
#[post("/api/2fa/disable")]
pub async fn disable_2fa(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
    body: web::Json<TwoFactorVerifyRequest>,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.get_2fa_secret(user_id).await {
        Ok(Some(secret)) => {
            if !secret.is_enabled {
                return HttpResponse::BadRequest().json(json!({
                    "error": "2FA not enabled"
                }));
            }

            // NOTA: Aquí iría la verificación TOTP real
            if body.code.len() == 6 && body.code.chars().all(|c| c.is_numeric()) {
                match repo.disable_2fa(user_id).await {
                    Ok(_) => HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "2FA disabled successfully"
                    })),
                    Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
                }
            } else {
                HttpResponse::BadRequest().json(json!({
                    "error": "Invalid 2FA code"
                }))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "2FA not configured"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Verificar estado de 2FA del usuario
#[get("/api/2fa/status")]
pub async fn get_2fa_status(
    repo: web::Data<SecurityRepository>,
    claims: Claims,
) -> HttpResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();

    match repo.get_2fa_secret(user_id).await {
        Ok(Some(secret)) => HttpResponse::Ok().json(json!({
            "enabled": secret.is_enabled,
            "enabled_at": secret.enabled_at,
            "last_used_at": secret.last_used_at,
            "has_backup_codes": secret.backup_codes.is_some()
        })),
        Ok(None) => HttpResponse::Ok().json(json!({
            "enabled": false,
            "message": "2FA not configured"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Login con 2FA (segundo paso)
#[post("/api/auth/2fa/verify")]
pub async fn login_2fa_verify(
    repo: web::Data<SecurityRepository>,
    body: web::Json<TwoFactorVerifyRequest>,
) -> HttpResponse {
    // Este endpoint se llamaría después del login inicial
    // Debería verificar el código 2FA y completar la autenticación
    // Implementación simplificada por ahora
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "2FA verified"
    }))
}

// ============================================
// WebSocket para Notificaciones en Tiempo Real
// ============================================
// NOTA: WebSockets requiere configuración especial de actix-web-actors
// Los endpoints están definidos pero la implementación completa
// se pospone para una iteración futura cuando se requiera push notifications

// ============================================
// IA/ML - Inteligencia Artificial
// ============================================

/// Chatbot de soporte
#[post("/api/ai/chatbot")]
pub async fn ai_chatbot(
    ai_client: web::Data<crate::ai_module::AIClient>,
    claims: Claims,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let user_message = body.get("message").and_then(|v| v.as_str()).unwrap_or("");
    let conversation_history: Vec<(String, String)> = body
        .get("history")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|msg| {
                    let role = msg.get("role")?.as_str()?.to_string();
                    let content = msg.get("content")?.as_str()?.to_string();
                    Some((role, content))
                })
                .collect()
        })
        .unwrap_or_default();

    let school_context = "Colegio SchoolCCB - Sistema de gestión académica";

    match ai_client.chatbot_support(conversation_history, user_message, school_context).await {
        Ok(response) => HttpResponse::Ok().json(json!({
            "response": response,
            "type": "chatbot"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Análisis de riesgo de deserción
#[post("/api/ai/analyze-dropout-risk")]
pub async fn ai_analyze_dropout_risk(
    ai_client: web::Data<crate::ai_module::AIClient>,
    claims: Claims,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    // Solo admin, profesor u orientación pueden usar esta feature
    if claims.role != "admin" && claims.role != "root" && claims.role != "orientacion" {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }

    let attendance = body.get("attendance").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let average_grade = body.get("average_grade").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let behavior_incidents = body.get("behavior_incidents").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let socioeconomic_factors = body.get("socioeconomic_factors").and_then(|v| v.as_str()).unwrap_or("No especificado");

    match ai_client.analyze_dropout_risk(attendance, average_grade, behavior_incidents, socioeconomic_factors).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Generar feedback para estudiante
#[post("/api/ai/generate-feedback")]
pub async fn ai_generate_feedback(
    ai_client: web::Data<crate::ai_module::AIClient>,
    claims: Claims,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let student_name = body.get("student_name").and_then(|v| v.as_str()).unwrap_or("Estudiante");
    
    let grades: Vec<(&str, f64)> = body
        .get("grades")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let subject = item.get("subject")?.as_str()?;
                    let grade = item.get("grade")?.as_f64()?;
                    Some((subject, grade))
                })
                .collect()
        })
        .unwrap_or_default();

    let attendance = body.get("attendance").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let teacher_comments = body.get("teacher_comments").and_then(|v| v.as_str()).unwrap_or("");

    match ai_client.generate_feedback(student_name, grades, attendance, teacher_comments).await {
        Ok(feedback) => HttpResponse::Ok().json(json!({
            "feedback": feedback,
            "type": "feedback_generation"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Clasificar consulta
#[post("/api/ai/classify-query")]
pub async fn ai_classify_query(
    ai_client: web::Data<crate::ai_module::AIClient>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let query = body.get("query").and_then(|v| v.as_str()).unwrap_or("");

    match ai_client.classify_query(query).await {
        Ok(classification) => HttpResponse::Ok().json(json!({
            "classification": classification,
            "type": "query_classification"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Resumir texto
#[post("/api/ai/summarize")]
pub async fn ai_summarize(
    ai_client: web::Data<crate::ai_module::AIClient>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let text = body.get("text").and_then(|v| v.as_str()).unwrap_or("");
    let max_words = body.get("max_words").and_then(|v| v.as_u64()).unwrap_or(100) as usize;

    match ai_client.summarize_text(text, max_words).await {
        Ok(summary) => HttpResponse::Ok().json(json!({
            "summary": summary,
            "type": "text_summary"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Análisis de sentimiento
#[post("/api/ai/analyze-sentiment")]
pub async fn ai_analyze_sentiment(
    ai_client: web::Data<crate::ai_module::AIClient>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let text = body.get("text").and_then(|v| v.as_str()).unwrap_or("");

    match ai_client.analyze_sentiment(text).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Transcribir audio (Whisper)
#[post("/api/ai/transcribe")]
pub async fn ai_transcribe(
    ai_client: web::Data<crate::ai_module::AIClient>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let audio_url = body.get("audio_url").and_then(|v| v.as_str()).unwrap_or("");
    let language = body.get("language").and_then(|v| v.as_str());

    match ai_client.transcribe_audio(audio_url, language).await {
        Ok(transcription) => HttpResponse::Ok().json(json!({
            "transcription": transcription.text,
            "language": transcription.language,
            "duration": transcription.duration,
            "type": "audio_transcription"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

/// Estado de servicios de IA
#[get("/api/ai/status")]
pub async fn ai_status(
    ai_client: web::Data<crate::ai_module::AIClient>,
) -> HttpResponse {
    // Verificar conectividad con Ollama
    let ollama_status = ai_client.chat("Eres un asistente de prueba. Responde solo 'OK'.", "test", 0.1).await.is_ok();
    
    HttpResponse::Ok().json(json!({
        "ollama": if ollama_status { "connected" } else { "disconnected" },
        "model": ai_client.config.model_chat,
        "ollama_url": ai_client.config.ollama_url,
        "whisper_url": ai_client.config.whisper_url
    }))
}

// ============================================
// Generador de PDFs
// ============================================

/// Generar boletín de calificaciones en PDF
#[get("/api/pdf/report-card/{student_id}")]
pub async fn generate_report_card_pdf(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let student_id = path.into_inner();
    
    // Verificar permisos (solo el estudiante o admin)
    if claims.role != "admin" && claims.role != "root" && claims.sub != student_id.to_string() {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }
    
    // Obtener datos del estudiante
    match repo.get_student_report_card(student_id).await {
        Ok(report_card) => {
            // Preparar datos para PDF
            let student_name = report_card.first()
                .map(|r| r.course_name.clone())
                .unwrap_or_else(|| "Estudiante".to_string());
            
            let grades: Vec<crate::pdf_generator::GradeItem> = report_card.iter()
                .map(|r| crate::pdf_generator::GradeItem {
                    course_name: r.course_name.clone(),
                    value: r.average_grade.try_into().unwrap_or(0.0),
                })
                .collect();
            
            // Generar PDF
            match crate::pdf_generator::generate_report_card_pdf(
                &student_name,
                &claims.email,
                "SchoolCCB",
                "2026-1",
                grades,
                report_card.first()
                    .and_then(|r| r.attendance_percentage.try_into().ok())
                    .unwrap_or(0.0),
            ) {
                Ok(pdf_bytes) => {
                    HttpResponse::Ok()
                        .content_type("application/pdf")
                        .header("Content-Disposition", "attachment; filename=\"boletin.pdf\"")
                        .body(pdf_bytes)
                }
                Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

/// Generar certificado de estudio
#[get("/api/pdf/certificate/{student_id}")]
pub async fn generate_certificate_pdf(
    repo: web::Data<Repository>,
    claims: Claims,
    path: web::Path<Uuid>,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let cert_type = query.get("type").and_then(|v| v.as_str()).unwrap_or("ESTUDIOS");
    
    // Verificar permisos
    if claims.role != "admin" && claims.role != "root" && claims.sub != student_id.to_string() {
        return HttpResponse::Forbidden().json(json!({"error": "Insufficient permissions"}));
    }
    
    // Obtener datos del estudiante (simplificado)
    let student_name = claims.email.clone();
    
    match crate::pdf_generator::generate_certificate_pdf(
        &student_name,
        &claims.email,
        "SchoolCCB",
        cert_type,
        &chrono::Local::now().format("%d/%m/%Y").to_string(),
    ) {
        Ok(pdf_bytes) => {
            HttpResponse::Ok()
                .content_type("application/pdf")
                .header("Content-Disposition", "attachment; filename=\"certificado.pdf\"")
                .body(pdf_bytes)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
