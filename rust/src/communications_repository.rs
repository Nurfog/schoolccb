// ============================================
// Repositorio de Comunicaciones (Fase 6)
// ============================================

use crate::models::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct CommunicationsRepository {
    pool: Pool<Postgres>,
}

impl CommunicationsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // ============================================
    // Notificaciones
    // ============================================

    /// Crear notificación
    pub async fn create_notification(
        &self,
        user_id: Uuid,
        title: &str,
        message: &str,
        notification_type: &str,
        data: Option<serde_json::Value>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Notification, sqlx::Error> {
        sqlx::query_as::<_, Notification>(
            r#"
            INSERT INTO notifications (user_id, title, message, type, data, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(title)
        .bind(message)
        .bind(notification_type)
        .bind(data)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener notificaciones de un usuario (paginadas)
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        sqlx::query_as::<_, Notification>(
            r#"
            SELECT * FROM notifications
            WHERE user_id = $1
              AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Contar notificaciones no leídas
    pub async fn count_unread_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM notifications
            WHERE user_id = $1
              AND is_read = FALSE
              AND (expires_at IS NULL OR expires_at > NOW())
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Marcar notificación como leída
    pub async fn mark_notification_read(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = TRUE,
                read_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND user_id = $2
            "#
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Marcar todas las notificaciones como leídas
    pub async fn mark_all_notifications_read(
        &self,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = TRUE,
                read_at = CURRENT_TIMESTAMP
            WHERE user_id = $1 AND is_read = FALSE
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Eliminar notificación
    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM notifications
            WHERE id = $1 AND user_id = $2
            "#
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ============================================
    // Preferencias de Notificación
    // ============================================

    /// Obtener o crear preferencias
    pub async fn get_or_create_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<NotificationPreference, sqlx::Error> {
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            INSERT INTO notification_preferences (user_id)
            VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Actualizar preferencias
    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        email_enabled: Option<bool>,
        push_enabled: Option<bool>,
        sms_enabled: Option<bool>,
        in_app_enabled: Option<bool>,
        categories: Option<serde_json::Value>,
        quiet_hours_enabled: Option<bool>,
        quiet_hours_start: Option<chrono::NaiveTime>,
        quiet_hours_end: Option<chrono::NaiveTime>,
    ) -> Result<NotificationPreference, sqlx::Error> {
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            UPDATE notification_preferences
            SET email_enabled = COALESCE($2, email_enabled),
                push_enabled = COALESCE($3, push_enabled),
                sms_enabled = COALESCE($4, sms_enabled),
                in_app_enabled = COALESCE($5, in_app_enabled),
                categories = COALESCE($6, categories),
                quiet_hours_enabled = COALESCE($7, quiet_hours_enabled),
                quiet_hours_start = COALESCE($8, quiet_hours_start),
                quiet_hours_end = COALESCE($9, quiet_hours_end),
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(email_enabled)
        .bind(push_enabled)
        .bind(sms_enabled)
        .bind(in_app_enabled)
        .bind(categories)
        .bind(quiet_hours_enabled)
        .bind(quiet_hours_start)
        .bind(quiet_hours_end)
        .fetch_one(&self.pool)
        .await
    }

    // ============================================
    // Plantillas
    // ============================================

    /// Obtener plantilla por código
    pub async fn get_template_by_code(
        &self,
        code: &str,
        school_id: Option<Uuid>,
    ) -> Result<NotificationTemplate, sqlx::Error> {
        // Primero busca plantilla del colegio, si no existe usa plantilla del sistema
        sqlx::query_as::<_, NotificationTemplate>(
            r#"
            SELECT * FROM notification_templates
            WHERE code = $1
              AND (school_id = $2 OR school_id IS NULL)
              AND is_active = TRUE
            ORDER BY school_id DESC NULLS LAST
            LIMIT 1
            "#
        )
        .bind(code)
        .bind(school_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Listar plantillas activas
    pub async fn list_templates(
        &self,
        school_id: Option<Uuid>,
        category: Option<&str>,
    ) -> Result<Vec<NotificationTemplate>, sqlx::Error> {
        let mut query = String::from(
            r#"
            SELECT * FROM notification_templates
            WHERE is_active = TRUE
              AND (school_id = $1 OR school_id IS NULL)
            "#
        );

        if let Some(_cat) = category {
            query.push_str(" AND category = $2");
        }

        query.push_str(" ORDER BY name ASC");

        let mut builder = sqlx::query_as::<_, NotificationTemplate>(&query);
        builder = builder.bind(school_id);
        
        if let Some(_) = category {
            builder = builder.bind(category);
        }

        builder.fetch_all(&self.pool).await
    }

    /// Actualizar plantilla (solo si no es del sistema)
    pub async fn update_template(
        &self,
        template_id: Uuid,
        subject: Option<&str>,
        body: Option<&str>,
        variables: Option<serde_json::Value>,
    ) -> Result<NotificationTemplate, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>(
            r#"
            UPDATE notification_templates
            SET subject = COALESCE($2, subject),
                body = COALESCE($3, body),
                variables = COALESCE($4, variables),
                version = version + 1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND is_system = FALSE
            RETURNING *
            "#
        )
        .bind(template_id)
        .bind(subject)
        .bind(body)
        .bind(variables)
        .fetch_one(&self.pool)
        .await
    }

    // ============================================
    // Comunicados
    // ============================================

    /// Crear comunicado
    pub async fn create_announcement(
        &self,
        school_id: Uuid,
        title: &str,
        content: &str,
        summary: Option<&str>,
        category: &str,
        target_audience: serde_json::Value,
        priority: i32,
        scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        allow_comments: bool,
        requires_confirmation: bool,
        attachment_urls: serde_json::Value,
        created_by: Uuid,
    ) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>(
            r#"
            INSERT INTO announcements (
                school_id, title, content, summary, category,
                target_audience, priority, scheduled_at, expires_at,
                allow_comments, requires_confirmation, attachment_urls, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#
        )
        .bind(school_id)
        .bind(title)
        .bind(content)
        .bind(summary)
        .bind(category)
        .bind(target_audience)
        .bind(priority)
        .bind(scheduled_at)
        .bind(expires_at)
        .bind(allow_comments)
        .bind(requires_confirmation)
        .bind(attachment_urls)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Publicar comunicado
    pub async fn publish_announcement(
        &self,
        announcement_id: Uuid,
        school_id: Uuid,
    ) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET is_published = TRUE,
                published_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND school_id = $2
            RETURNING *
            "#
        )
        .bind(announcement_id)
        .bind(school_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener comunicado por ID
    pub async fn get_announcement(
        &self,
        announcement_id: Uuid,
    ) -> Result<Option<AnnouncementWithAuthor>, sqlx::Error> {
        sqlx::query_as::<_, AnnouncementWithAuthor>(
            r#"
            SELECT a.*, u.name as author_name
            FROM announcements a
            JOIN users u ON a.created_by = u.id
            WHERE a.id = $1
            "#
        )
        .bind(announcement_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Listar comunicados publicados (paginados)
    pub async fn list_published_announcements(
        &self,
        school_id: Uuid,
        category: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AnnouncementWithAuthor>, sqlx::Error> {
        let mut query = String::from(
            r#"
            SELECT a.*, u.name as author_name
            FROM announcements a
            JOIN users u ON a.created_by = u.id
            WHERE a.school_id = $1
              AND a.is_published = TRUE
              AND (a.scheduled_at IS NULL OR a.scheduled_at <= NOW())
              AND (a.expires_at IS NULL OR a.expires_at > NOW())
            "#
        );

        if let Some(_cat) = category {
            query.push_str(" AND a.category = $3");
        }

        query.push_str(" ORDER BY a.priority DESC, a.published_at DESC LIMIT $2 OFFSET $3");

        let mut builder = sqlx::query_as::<_, AnnouncementWithAuthor>(&query);
        builder = builder.bind(school_id);
        
        if let Some(_) = category {
            builder = builder.bind(category);
            builder = builder.bind(limit);
            builder = builder.bind(offset);
        } else {
            builder = builder.bind(limit);
            builder = builder.bind(offset);
        }

        builder.fetch_all(&self.pool).await
    }

    /// Actualizar comunicado
    pub async fn update_announcement(
        &self,
        announcement_id: Uuid,
        school_id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
        summary: Option<&str>,
        category: Option<&str>,
        target_audience: Option<serde_json::Value>,
        priority: Option<i32>,
        scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        allow_comments: Option<bool>,
        requires_confirmation: Option<bool>,
        attachment_urls: Option<serde_json::Value>,
        updated_by: Uuid,
    ) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET title = COALESCE($4, title),
                content = COALESCE($5, content),
                summary = COALESCE($6, summary),
                category = COALESCE($7, category),
                target_audience = COALESCE($8, target_audience),
                priority = COALESCE($9, priority),
                scheduled_at = COALESCE($10, scheduled_at),
                expires_at = COALESCE($11, expires_at),
                allow_comments = COALESCE($12, allow_comments),
                requires_confirmation = COALESCE($13, requires_confirmation),
                attachment_urls = COALESCE($14, attachment_urls),
                updated_by = $15,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND school_id = $2
            RETURNING *
            "#
        )
        .bind(announcement_id)
        .bind(school_id)
        .bind(title)
        .bind(content)
        .bind(summary)
        .bind(category)
        .bind(target_audience)
        .bind(priority)
        .bind(scheduled_at)
        .bind(expires_at)
        .bind(allow_comments)
        .bind(requires_confirmation)
        .bind(attachment_urls)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Eliminar comunicado
    pub async fn delete_announcement(
        &self,
        announcement_id: Uuid,
        school_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM announcements
            WHERE id = $1 AND school_id = $2
            "#
        )
        .bind(announcement_id)
        .bind(school_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ============================================
    // Lecturas de Comunicados
    // ============================================

    /// Registrar lectura de comunicado
    pub async fn record_announcement_reading(
        &self,
        announcement_id: Uuid,
        user_id: Uuid,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO announcement_readings (announcement_id, user_id, ip_address, user_agent)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (announcement_id, user_id) DO UPDATE
            SET read_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(announcement_id)
        .bind(user_id)
        .bind(ip_address)
        .bind(user_agent)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Confirmar lectura de comunicado
    pub async fn confirm_announcement_reading(
        &self,
        announcement_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE announcement_readings
            SET is_confirmed = TRUE
            WHERE announcement_id = $1 AND user_id = $2
            "#
        )
        .bind(announcement_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Obtener estadísticas de comunicado
    pub async fn get_announcement_stats(
        &self,
        announcement_id: Uuid,
    ) -> Result<Option<AnnouncementStats>, sqlx::Error> {
        sqlx::query_as::<_, AnnouncementStats>(
            r#"
            SELECT * FROM announcement_stats
            WHERE announcement_id = $1
            "#
        )
        .bind(announcement_id)
        .fetch_optional(&self.pool)
        .await
    }

    // ============================================
    // Justificaciones de Inasistencia
    // ============================================

    /// Crear justificación
    pub async fn create_justification(
        &self,
        student_id: Uuid,
        parent_id: Uuid,
        school_id: Uuid,
        absence_date: chrono::NaiveDate,
        absence_type: &str,
        start_time: Option<chrono::NaiveTime>,
        end_time: Option<chrono::NaiveTime>,
        reason: &str,
        attachment_urls: serde_json::Value,
    ) -> Result<AttendanceJustification, sqlx::Error> {
        sqlx::query_as::<_, AttendanceJustification>(
            r#"
            INSERT INTO attendance_justifications (
                student_id, parent_id, school_id, absence_date,
                absence_type, start_time, end_time, reason, attachment_urls
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(student_id)
        .bind(parent_id)
        .bind(school_id)
        .bind(absence_date)
        .bind(absence_type)
        .bind(start_time)
        .bind(end_time)
        .bind(reason)
        .bind(attachment_urls)
        .fetch_one(&self.pool)
        .await
    }

    /// Obtener justificaciones de un estudiante
    pub async fn get_student_justifications(
        &self,
        student_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AttendanceJustificationWithDetails>, sqlx::Error> {
        sqlx::query_as::<_, AttendanceJustificationWithDetails>(
            r#"
            SELECT 
                aj.*,
                s.name as student_name,
                p.name as parent_name,
                r.name as reviewer_name
            FROM attendance_justifications aj
            JOIN users s ON aj.student_id = s.id
            JOIN users p ON aj.parent_id = p.id
            LEFT JOIN users r ON aj.reviewed_by = r.id
            WHERE aj.student_id = $1
            ORDER BY aj.absence_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(student_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Obtener justificaciones pendientes de un colegio
    pub async fn get_pending_justifications(
        &self,
        school_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AttendanceJustificationWithDetails>, sqlx::Error> {
        sqlx::query_as::<_, AttendanceJustificationWithDetails>(
            r#"
            SELECT 
                aj.*,
                s.name as student_name,
                p.name as parent_name,
                r.name as reviewer_name
            FROM attendance_justifications aj
            JOIN users s ON aj.student_id = s.id
            JOIN users p ON aj.parent_id = p.id
            LEFT JOIN users r ON aj.reviewed_by = r.id
            WHERE aj.school_id = $1 AND aj.status = 'pending'
            ORDER BY aj.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(school_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Revisar justificación
    pub async fn review_justification(
        &self,
        justification_id: Uuid,
        school_id: Uuid,
        status: &str,
        review_notes: Option<&str>,
        reviewed_by: Uuid,
    ) -> Result<AttendanceJustification, sqlx::Error> {
        sqlx::query_as::<_, AttendanceJustification>(
            r#"
            UPDATE attendance_justifications
            SET status = $4,
                review_notes = $5,
                reviewed_by = $6,
                reviewed_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND school_id = $2 AND status = 'pending'
            RETURNING *
            "#
        )
        .bind(justification_id)
        .bind(school_id)
        .bind(status)
        .bind(review_notes)
        .bind(reviewed_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Contar justificaciones pendientes
    pub async fn count_pending_justifications(
        &self,
        school_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM attendance_justifications
            WHERE school_id = $1 AND status = 'pending'
            "#
        )
        .bind(school_id)
        .fetch_one(&self.pool)
        .await
    }
}
