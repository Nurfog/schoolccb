-- ============================================
-- Migración: Índices y Mejoras de Rendimiento
-- ============================================

-- Índices para users (búsquedas frecuentes por email y school_id)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_school_id ON users(school_id);
CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_users_school_role ON users(school_id, role_id);

-- Índices para schools (búsqueda por subdomain para multi-tenancy)
CREATE INDEX IF NOT EXISTS idx_schools_subdomain ON schools(subdomain);
CREATE INDEX IF NOT EXISTS idx_schools_country_id ON schools(country_id);

-- Índices para courses
CREATE INDEX IF NOT EXISTS idx_courses_school_id ON courses(school_id);
CREATE INDEX IF NOT EXISTS idx_courses_teacher_id ON courses(teacher_id);

-- Índices para enrollments (tablas de relación)
CREATE INDEX IF NOT EXISTS idx_enrollments_student_id ON enrollments(student_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_course_id ON enrollments(course_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_student_course ON enrollments(student_id, course_id);

-- Índices para grades (consultas por curso y estudiante)
CREATE INDEX IF NOT EXISTS idx_grades_student_id ON grades(student_id);
CREATE INDEX IF NOT EXISTS idx_grades_course_id ON grades(course_id);
CREATE INDEX IF NOT EXISTS idx_grades_student_course ON grades(student_id, course_id);

-- Índices para attendance (consultas por fecha y curso)
CREATE INDEX IF NOT EXISTS idx_attendance_student_id ON attendance(student_id);
CREATE INDEX IF NOT EXISTS idx_attendance_course_id ON attendance(course_id);
CREATE INDEX IF NOT EXISTS idx_attendance_date ON attendance(date);
CREATE INDEX IF NOT EXISTS idx_attendance_student_course_date ON attendance(student_id, course_id, date);

-- Índices para academic_periods
CREATE INDEX IF NOT EXISTS idx_academic_periods_school_id ON academic_periods(school_id);
CREATE INDEX IF NOT EXISTS idx_academic_periods_is_active ON academic_periods(school_id, is_active);

-- Índices para saas_licenses (monitoreo de licencias por vencer)
CREATE INDEX IF NOT EXISTS idx_saas_licenses_school_id ON saas_licenses(school_id);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_status ON saas_licenses(status);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_expiry_date ON saas_licenses(expiry_date);
CREATE INDEX IF NOT EXISTS idx_saas_licenses_status_expiry ON saas_licenses(status, expiry_date);

-- Índices para role_permissions (RBAC)
CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_permission_id ON role_permissions(permission_id);

-- Índices para teachers y students (tablas de extensión de usuarios)
CREATE INDEX IF NOT EXISTS idx_teachers_user_id ON teachers(user_id);
CREATE INDEX IF NOT EXISTS idx_students_user_id ON students(user_id);

-- Mejorar foreign keys con ON DELETE CASCADE donde sea apropiado
ALTER TABLE enrollments 
    DROP CONSTRAINT IF EXISTS enrollments_student_id_fkey,
    ADD CONSTRAINT enrollments_student_id_fkey 
        FOREIGN KEY (student_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE enrollments 
    DROP CONSTRAINT IF EXISTS enrollments_course_id_fkey,
    ADD CONSTRAINT enrollments_course_id_fkey 
        FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE;

ALTER TABLE grades 
    DROP CONSTRAINT IF EXISTS grades_student_id_fkey,
    ADD CONSTRAINT grades_student_id_fkey 
        FOREIGN KEY (student_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE grades 
    DROP CONSTRAINT IF EXISTS grades_course_id_fkey,
    ADD CONSTRAINT grades_course_id_fkey 
        FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE;

ALTER TABLE attendance 
    DROP CONSTRAINT IF EXISTS attendance_student_id_fkey,
    ADD CONSTRAINT attendance_student_id_fkey 
        FOREIGN KEY (student_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE attendance 
    DROP CONSTRAINT IF EXISTS attendance_course_id_fkey,
    ADD CONSTRAINT attendance_course_id_fkey 
        FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE;

ALTER TABLE teachers 
    DROP CONSTRAINT IF EXISTS teachers_user_id_fkey,
    ADD CONSTRAINT teachers_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE students 
    DROP CONSTRAINT IF EXISTS students_user_id_fkey,
    ADD CONSTRAINT students_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE courses 
    DROP CONSTRAINT IF EXISTS courses_school_id_fkey,
    ADD CONSTRAINT courses_school_id_fkey 
        FOREIGN KEY (school_id) REFERENCES schools(id) ON DELETE CASCADE;

ALTER TABLE users 
    DROP CONSTRAINT IF EXISTS users_school_id_fkey,
    ADD CONSTRAINT users_school_id_fkey 
        FOREIGN KEY (school_id) REFERENCES schools(id) ON DELETE CASCADE;
