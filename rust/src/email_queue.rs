// ============================================
// Email Queue con Redis y Reintentos Automáticos
// Fase 6 - Sistema de Notificaciones
// ============================================

use lettre::message::header::ContentType;
use lettre::message::{Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use redis::aio::ConnectionManager;
use redis::{Client as RedisClient, RedisError};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error, debug};

/// Configuración del servicio de email
#[derive(Clone, Debug)]
pub struct EmailService {
    smtp_config: Option<SmtpConfig>,
    redis_client: Option<RedisClient>,
    queue_name: String,
    max_retries: u32,
}

#[derive(Clone, Debug)]
struct SmtpConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    from_name: String,
    from_email: String,
}

/// Email encolado para envío asíncrono
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedEmail {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: Option<String>,
    pub retry_count: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email_type: EmailType,
}

/// Tipo de email para clasificación y plantillas
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Welcome,
    PasswordReset,
    Notification,
    Announcement,
    GradePublished,
    AttendanceAlert,
    PaymentReminder,
    Custom,
}

impl QueuedEmail {
    pub fn new(
        to: String,
        subject: String,
        html_body: String,
        text_body: Option<String>,
        email_type: EmailType,
    ) -> Self {
        Self {
            to,
            subject,
            html_body,
            text_body,
            retry_count: 0,
            created_at: chrono::Utc::now(),
            email_type,
        }
    }
}

impl EmailService {
    /// Crear nuevo servicio de email
    pub async fn new() -> Result<Self, String> {
        let smtp_config = Self::load_smtp_config();
        let redis_client = Self::load_redis_client();
        let queue_name = env::var("EMAIL_QUEUE_NAME").unwrap_or_else(|_| "email_queue".to_string());
        let max_retries = env::var("EMAIL_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3u32);

        if smtp_config.is_some() {
            info!("Email service configured");
        } else {
            warn!("Email service not configured (SMTP disabled)");
        }

        if redis_client.is_some() {
            info!("Redis queue configured for async email delivery");
        } else {
            warn!("Redis not configured, emails will be sent synchronously");
        }

        Ok(Self {
            smtp_config,
            redis_client,
            queue_name,
            max_retries,
        })
    }

    fn load_smtp_config() -> Option<SmtpConfig> {
        let enabled: bool = env::var("SMTP_ENABLED").ok()?.parse().ok()?;
        if !enabled {
            return None;
        }

        Some(SmtpConfig {
            host: env::var("SMTP_HOST").ok()?,
            port: env::var("SMTP_PORT").ok()?.parse().ok()?,
            username: env::var("SMTP_USER").ok()?,
            password: env::var("SMTP_PASSWORD").ok()?,
            from_name: env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "SchoolCCB".to_string()),
            from_email: env::var("SMTP_FROM_EMAIL").ok()?,
        })
    }

    fn load_redis_client() -> Option<RedisClient> {
        let redis_url = env::var("REDIS_URL").ok()?;
        RedisClient::open(redis_url.as_str()).ok()
    }

    /// Enviar email inmediatamente (síncrono)
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<(), String> {
        let config = self.smtp_config.as_ref().ok_or("SMTP not configured")?;

        let email = self.build_email(to, subject, html_body, text_body, config)?;
        let mailer = self.build_mailer(config)?;

        match mailer.send(email).await {
            Ok(_) => {
                info!(to = %to, subject = %subject, "Email sent successfully");
                Ok(())
            }
            Err(e) => {
                error!(to = %to, error = %e, "Failed to send email");
                Err(format!("Failed to send email: {}", e))
            }
        }
    }

    /// Encolar email para envío asíncrono con Redis
    pub async fn queue_email(&self, queued_email: QueuedEmail) -> Result<(), String> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client
                .get_tokio_connection_manager()
                .await
                .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

            let email_json = serde_json::to_string(&queued_email)
                .map_err(|e| format!("Failed to serialize email: {}", e))?;

            redis::cmd("LPUSH")
                .arg(&self.queue_name)
                .arg(&email_json)
                .query_async::<()>(&mut conn)
                .await
                .map_err(|e| format!("Failed to push to queue: {}", e))?;

            debug!(to = %queued_email.to, "Email queued successfully");
            Ok(())
        } else {
            // Si no hay Redis, enviar inmediatamente
            warn!("Redis not configured, sending email synchronously");
            self.send_email(
                &queued_email.to,
                &queued_email.subject,
                &queued_email.html_body,
                queued_email.text_body.as_deref(),
            )
            .await
        }
    }

    /// Procesar cola de emails (worker)
    pub async fn process_email_queue(&self) -> Result<(), String> {
        if self.smtp_config.is_none() {
            debug!("SMTP not configured, skipping queue processing");
            return Ok(());
        }

        if self.redis_client.is_none() {
            debug!("Redis not configured, skipping queue processing");
            return Ok(());
        }

        let client = self.redis_client.as_ref().unwrap();
        let mut conn = client
            .get_tokio_connection_manager()
            .await
            .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

        // Intentar obtener un email de la cola
        let result: Option<String> = redis::cmd("RPOP")
            .arg(&self.queue_name)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("Failed to pop from queue: {}", e))?;

        if let Some(email_json) = result {
            match serde_json::from_str::<QueuedEmail>(&email_json) {
                Ok(mut queued_email) => {
                    // Intentar enviar
                    match self.send_email(
                        &queued_email.to,
                        &queued_email.subject,
                        &queued_email.html_body,
                        queued_email.text_body.as_deref(),
                    ).await {
                        Ok(_) => {
                            info!(to = %queued_email.to, "Queued email sent successfully");
                        }
                        Err(e) => {
                            warn!(to = %queued_email.to, error = %e, "Failed to send queued email");
                            queued_email.retry_count += 1;

                            if queued_email.retry_count < self.max_retries {
                                // Re-encolar con backoff exponencial
                                let delay = Duration::from_secs(2u64.pow(queued_email.retry_count));
                                sleep(delay).await;

                                // Re-encolar al inicio de la cola (prioridad)
                                let email_json = serde_json::to_string(&queued_email)
                                    .map_err(|e| format!("Failed to serialize email: {}", e))?;

                                redis::cmd("LPUSH")
                                    .arg(&self.queue_name)
                                    .arg(&email_json)
                                    .query_async::<()>(&mut conn)
                                    .await
                                    .map_err(|e| format!("Failed to re-queue email: {}", e))?;

                                warn!(to = %queued_email.to, retry = queued_email.retry_count, "Email re-queued for retry");
                            } else {
                                error!(to = %queued_email.to, "Email failed after {} retries, moving to dead letter queue", self.max_retries);
                                // Mover a dead letter queue
                                let _ = self.move_to_dead_letter(&queued_email).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to deserialize queued email");
                }
            }
        }

        Ok(())
    }

    /// Mover email fallido a dead letter queue
    async fn move_to_dead_letter(&self, queued_email: &QueuedEmail) -> Result<(), String> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client
                .get_tokio_connection_manager()
                .await
                .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

            let email_json = serde_json::to_string(&queued_email)
                .map_err(|e| format!("Failed to serialize email: {}", e))?;

            redis::cmd("LPUSH")
                .arg("email_dead_letter_queue")
                .arg(&email_json)
                .query_async::<()>(&mut conn)
                .await
                .map_err(|e| format!("Failed to push to dead letter queue: {}", e))?;

            Ok(())
        } else {
            Ok(())
        }
    }

    /// Obtener longitud de la cola
    pub async fn get_queue_length(&self) -> Result<usize, String> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client
                .get_tokio_connection_manager()
                .await
                .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

            let length: usize = redis::cmd("LLEN")
                .arg(&self.queue_name)
                .query_async(&mut conn)
                .await
                .map_err(|e| format!("Failed to get queue length: {}", e))?;

            Ok(length)
        } else {
            Ok(0)
        }
    }

    /// Construir email
    fn build_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
        config: &SmtpConfig,
    ) -> Result<Message, String> {
        let mut email_builder = Message::builder()
            .from(format!("{} <{}>", config.from_name, config.from_email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject);

        let email = if let Some(text) = text_body {
            email_builder
                .multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_PLAIN)
                                .body(text.to_string())
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_HTML)
                                .body(html_body.to_string())
                        )
                )
                .unwrap()
        } else {
            email_builder
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string())
                .unwrap()
        };

        Ok(email)
    }

    /// Construir mailer SMTP
    fn build_mailer(&self, config: &SmtpConfig) -> Result<AsyncSmtpTransport<Tokio1Executor>, String> {
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
            .map_err(|e| format!("Failed to create SMTP relay: {}", e))?
            .port(config.port)
            .credentials(Credentials::new(
                config.username.clone(),
                config.password.clone(),
            ))
            .build();

        Ok(mailer)
    }

    // ============================================
    // Templates de Emails Predefinidos
    // ============================================

    pub async fn send_welcome_email(&self, to: &str, name: &str, school_name: &str) -> Result<(), String> {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center; }}
                .content {{ padding: 30px; background: #f9f9f9; }}
                .button {{ display: inline-block; padding: 12px 30px; background: #667eea; color: white; text-decoration: none; border-radius: 5px; }}
                .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>¡Bienvenido a {}!</h1>
                </div>
                <div class="content">
                    <p>Hola {},</p>
                    <p>Tu cuenta ha sido creada exitosamente en la plataforma de <strong>{}</strong>.</p>
                    <p>Ahora puedes acceder a todas las funcionalidades de la plataforma para gestionar tu experiencia académica.</p>
                    <p style="text-align: center; margin: 30px 0;">
                        <a href="https://app.schoolccb.com/login" class="button">Acceder a la Plataforma</a>
                    </p>
                    <p>Si tienes alguna pregunta, no dudes en contactar a nuestro equipo de soporte.</p>
                    <p>¡Saludos!<br>El equipo de SchoolCCB</p>
                </div>
                <div class="footer">
                    <p>© 2026 SchoolCCB. Todos los derechos reservados.</p>
                </div>
            </div>
        </body>
        </html>
        "#, school_name, name, school_name);

        let text = format!("Hola {},\n\nBienvenido a {}! Tu cuenta ha sido creada exitosamente.\n\nAccede en: https://app.schoolccb.com/login\n\nSaludos,\nEl equipo de SchoolCCB", name, school_name);

        let queued_email = QueuedEmail::new(
            to.to_string(),
            "¡Bienvenido a SchoolCCB!".to_string(),
            html,
            Some(text),
            EmailType::Welcome,
        );

        self.queue_email(queued_email).await
    }

    pub async fn send_notification_email(&self, to: &str, subject: &str, message: &str, link: Option<&str>) -> Result<(), String> {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; }}
                .content {{ padding: 30px; background: #f9f9f9; }}
                .notification {{ background: white; border-left: 4px solid #667eea; padding: 20px; margin: 20px 0; }}
                .button {{ display: inline-block; padding: 10px 25px; background: #667eea; color: white; text-decoration: none; border-radius: 5px; margin-top: 15px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>📬 Nueva Notificación</h1>
                </div>
                <div class="content">
                    <div class="notification">
                        <p style="margin: 0;">{}</p>
                        {}
                    </div>
                    <p>Saludos,<br>El equipo de SchoolCCB</p>
                </div>
            </div>
        </body>
        </html>
        "#, message, link.map_or(String::new(), |l| format!("<a href=\"{}\" class=\"button\">Ver más</a>", l)));

        let queued_email = QueuedEmail::new(
            to.to_string(),
            subject.to_string(),
            html,
            Some(message.to_string()),
            EmailType::Notification,
        );

        self.queue_email(queued_email).await
    }

    pub async fn send_announcement_email(
        &self,
        to: &str,
        announcement_title: &str,
        announcement_content: &str,
        school_name: &str,
    ) -> Result<(), String> {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; }}
                .content {{ padding: 30px; background: #f9f9f9; }}
                .announcement {{ background: white; padding: 20px; margin: 20px 0; border-radius: 5px; }}
                .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>📢 Comunicado de {}</h1>
                </div>
                <div class="content">
                    <div class="announcement">
                        <h2>{}</h2>
                        <p>{}</p>
                    </div>
                    <p>Para más detalles, inicia sesión en la plataforma.</p>
                    <p>Saludos,<br>El equipo de {}</p>
                </div>
                <div class="footer">
                    <p>© 2026 SchoolCCB. Todos los derechos reservados.</p>
                </div>
            </div>
        </body>
        </html>
        "#, school_name, announcement_title, announcement_content, school_name);

        let text = format!("Comunicado de {}\n\n{}\n\n{}", school_name, announcement_title, announcement_content);

        let queued_email = QueuedEmail::new(
            to.to_string(),
            format!("📢 Comunicado: {}", announcement_title),
            html,
            Some(text),
            EmailType::Announcement,
        );

        self.queue_email(queued_email).await
    }

    pub async fn send_grade_published_email(
        &self,
        to: &str,
        student_name: &str,
        course_name: &str,
        evaluation_name: &str,
        grade: &str,
    ) -> Result<(), String> {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; text-align: center; }}
                .content {{ padding: 30px; background: #f9f9f9; }}
                .grade {{ background: white; padding: 20px; margin: 20px 0; border-radius: 5px; text-align: center; }}
                .grade-value {{ font-size: 36px; font-weight: bold; color: #667eea; }}
                .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>📊 Nueva Calificación</h1>
                </div>
                <div class="content">
                    <p>Hola {},</p>
                    <p>Se ha publicado una nueva calificación:</p>
                    <div class="grade">
                        <p style="margin: 0; color: #666;">{}</p>
                        <p class="grade-value">{}</p>
                        <p style="margin: 0; color: #666;">{}</p>
                    </div>
                    <p>Revisa tu boletín para más detalles.</p>
                    <p>Saludos,<br>El equipo de SchoolCCB</p>
                </div>
                <div class="footer">
                    <p>© 2026 SchoolCCB. Todos los derechos reservados.</p>
                </div>
            </div>
        </body>
        </html>
        "#, student_name, evaluation_name, grade, course_name);

        let text = format!("Nueva calificación: {} - {} - {}", evaluation_name, grade, course_name);

        let queued_email = QueuedEmail::new(
            to.to_string(),
            format!("📊 Calificación Publicada: {}", course_name),
            html,
            Some(text),
            EmailType::GradePublished,
        );

        self.queue_email(queued_email).await
    }
}

impl Default for EmailService {
    fn default() -> Self {
        // En default, creamos una instancia vacía
        Self {
            smtp_config: None,
            redis_client: None,
            queue_name: "email_queue".to_string(),
            max_retries: 3,
        }
    }
}
