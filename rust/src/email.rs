use lettre::message::header::ContentType;
use lettre::message::{Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use std::env;
use tracing::{info, error};

#[derive(Clone, Debug)]
pub struct EmailService {
    smtp_config: Option<SmtpConfig>,
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

impl EmailService {
    pub fn new() -> Self {
        let smtp_config = Self::load_smtp_config();
        
        if smtp_config.is_some() {
            info!("Email service configured");
        } else {
            info!("Email service not configured (SMTP disabled)");
        }
        
        Self { smtp_config }
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
    
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<(), String> {
        let config = self.smtp_config.as_ref()
            .ok_or("SMTP not configured")?;
        
        // Crear email
        let mut email_builder = Message::builder()
            .from(format!("{} <{}>", config.from_name, config.from_email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject);
        
        // Agregar cuerpo
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
        
        // Configurar transporte SMTP
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
            .unwrap()
            .port(config.port)
            .credentials(Credentials::new(
                config.username.clone(),
                config.password.clone(),
            ))
            .build();
        
        // Enviar email
        match mailer.send(email).await {
            Ok(_) => {
                info!("Email sent to {}", to);
                Ok(())
            }
            Err(e) => {
                error!("Failed to send email: {}", e);
                Err(format!("Failed to send email: {}", e))
            }
        }
    }
    
    // Templates de emails predefinidos
    
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
        
        self.send_email(to, "¡Bienvenido a SchoolCCB!", &html, Some(&text)).await
    }
    
    pub async fn send_password_reset_email(&self, to: &str, name: &str, reset_link: &str) -> Result<(), String> {
        let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: #f6ad55; color: white; padding: 20px; text-align: center; }}
                .content {{ padding: 30px; background: #f9f9f9; }}
                .button {{ display: inline-block; padding: 12px 30px; background: #f6ad55; color: white; text-decoration: none; border-radius: 5px; }}
                .warning {{ background: #fef3c7; border-left: 4px solid #f6ad55; padding: 15px; margin: 20px 0; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🔐 Restablecer Contraseña</h1>
                </div>
                <div class="content">
                    <p>Hola {},</p>
                    <p>Has solicitado restablecer tu contraseña. Haz clic en el botón siguiente para continuar:</p>
                    <p style="text-align: center; margin: 30px 0;">
                        <a href="{}" class="button">Restablecer Contraseña</a>
                    </p>
                    <div class="warning">
                        <strong>⚠️ Importante:</strong> Este enlace expirará en 1 hora. Si no solicitaste este cambio, puedes ignorar este email.
                    </div>
                    <p>Saludos,<br>El equipo de SchoolCCB</p>
                </div>
            </div>
        </body>
        </html>
        "#, name, reset_link);
        
        self.send_email(to, "Restablecer contraseña - SchoolCCB", &html, Some("Haz clic en el enlace para restablecer tu contraseña.")).await
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
        
        self.send_email(to, subject, &html, Some(message)).await
    }
}

// Implementar Default para EmailService
impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}
