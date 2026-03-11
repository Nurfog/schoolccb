use serde::{Deserialize, Serialize};

// ============================================
// Plan Types and Feature Flags
// ============================================

/// Tipos de planes disponibles para los colegios
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PlanType {
    /// Plan básico - funcionalidades esenciales
    Basic,
    /// Plan premium - funcionalidades avanzadas
    Premium,
    /// Plan enterprise - todas las funcionalidades
    Enterprise,
}

impl PlanType {
    /// Verifica si el plan incluye una feature específica
    pub fn has_feature(&self, feature: &FeatureType) -> bool {
        match self {
            PlanType::Basic => matches!(
                feature,
                FeatureType::AcademicCore
                    | FeatureType::BasicReports
                    | FeatureType::CsvImport
                    | FeatureType::Branding
            ),
            PlanType::Premium => matches!(
                feature,
                FeatureType::AcademicCore
                    | FeatureType::BasicReports
                    | FeatureType::CsvImport
                    | FeatureType::Branding
                    | FeatureType::FinancialModule
                    | FeatureType::PdfGeneration
                    | FeatureType::EmailNotifications
                    | FeatureType::ParentPortal
            ),
            PlanType::Enterprise => true, // Enterprise incluye todo
        }
    }

    /// Límite de estudiantes por plan
    pub fn max_students(&self) -> Option<usize> {
        match self {
            PlanType::Basic => Some(500),
            PlanType::Premium => Some(2000),
            PlanType::Enterprise => None, // Ilimitado
        }
    }

    /// Límite de usuarios totales por plan
    pub fn max_users(&self) -> Option<usize> {
        match self {
            PlanType::Basic => Some(50),
            PlanType::Premium => Some(200),
            PlanType::Enterprise => None, // Ilimitado
        }
    }

    /// Precio mensual en USD
    pub fn monthly_price_usd(&self) -> rust_decimal::Decimal {
        match self {
            PlanType::Basic => rust_decimal::Decimal::from(49),
            PlanType::Premium => rust_decimal::Decimal::from(99),
            PlanType::Enterprise => rust_decimal::Decimal::from(249),
        }
    }
}

/// Tipos de features/modulos del sistema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureType {
    // === CORE (Todos los planes) ===
    AcademicCore,           // Gestión de cursos, estudiantes, profesores
    BasicReports,           // Reportes básicos de notas y asistencia
    CsvImport,              // Importación masiva CSV
    Branding,               // Personalización de marca

    // === PREMIUM ===
    FinancialModule,        // Módulo financiero y pagos
    PdfGeneration,          // Generación de PDFs (boletines, certificados)
    EmailNotifications,     // Notificaciones por email
    ParentPortal,           // Portal para padres

    // === ENTERPRISE ===
    SmsNotifications,       // Notificaciones SMS
    PushNotifications,      // Push notifications
    AdvancedAnalytics,      // Analítica avanzada y BI
    ApiAccess,              // Acceso completo a API
    AuditLogs,              // Logs de auditoría
    TwoFactorAuth,          // Autenticación 2FA
    CustomIntegrations,     // Integraciones personalizadas
    PrioritySupport,        // Soporte prioritario
    WhiteLabel,             // White label completo
    MultiCampus,            // Multi-sede
}

/// Estado de una feature para un colegio específico
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureStatus {
    pub feature: FeatureType,
    pub enabled: bool,
    pub limit: Option<i64>,
    pub used: Option<i64>,
}

// ============================================
// Plan Comparison Data
// ============================================

/// Información completa de un plan para mostrar en UI
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanInfo {
    pub name: String,
    pub price_monthly_usd: rust_decimal::Decimal,
    pub price_yearly_usd: rust_decimal::Decimal,
    pub max_students: Option<usize>,
    pub max_users: Option<usize>,
    pub features: Vec<FeatureInfo>,
    pub popular: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureInfo {
    pub name: String,
    pub description: String,
    pub included: bool,
}

impl PlanType {
    /// Obtener información detallada del plan para mostrar en UI
    pub fn get_plan_info(&self) -> PlanInfo {
        let all_features = vec![
            (FeatureType::AcademicCore, "Gestión Académica", "Cursos, estudiantes, profesores, matrículas"),
            (FeatureType::BasicReports, "Reportes Básicos", "Notas y asistencia básica"),
            (FeatureType::CsvImport, "Importación CSV", "Carga masiva de usuarios"),
            (FeatureType::Branding, "Personalización", "Logo y colores del colegio"),
            (FeatureType::FinancialModule, "Módulo Financiero", "Pagos, pensiones, morosidad"),
            (FeatureType::PdfGeneration, "Generación PDF", "Boletines, certificados oficiales"),
            (FeatureType::EmailNotifications, "Email Notifications", "Alertas y comunicados por email"),
            (FeatureType::ParentPortal, "Portal para Padres", "Vista para acudientes"),
            (FeatureType::SmsNotifications, "SMS Notifications", "Alertas por SMS"),
            (FeatureType::PushNotifications, "Push Notifications", "Notificaciones push móvil"),
            (FeatureType::AdvancedAnalytics, "Analítica Avanzada", "Dashboards BI y métricas"),
            (FeatureType::ApiAccess, "API Access", "Acceso completo a API REST"),
            (FeatureType::AuditLogs, "Audit Logs", "Registro detallado de acciones"),
            (FeatureType::TwoFactorAuth, "2FA", "Autenticación multi-factor"),
            (FeatureType::CustomIntegrations, "Integraciones", "Conexión con sistemas externos"),
            (FeatureType::PrioritySupport, "Soporte Prioritario", "Atención prioritaria 24/7"),
            (FeatureType::WhiteLabel, "White Label", "Marca completamente personalizada"),
            (FeatureType::MultiCampus, "Multi-Sede", "Gestión de múltiples campus"),
        ];

        let features = all_features.iter().map(|(feature, name, desc)| {
            FeatureInfo {
                name: name.to_string(),
                description: desc.to_string(),
                included: self.has_feature(feature),
            }
        }).collect();

        let yearly_price = self.monthly_price_usd() * rust_decimal::Decimal::from(10); // 2 meses gratis

        PlanInfo {
            name: format!("{:?}", self),
            price_monthly_usd: self.monthly_price_usd(),
            price_yearly_usd: yearly_price,
            max_students: self.max_students(),
            max_users: self.max_users(),
            features,
            popular: *self == PlanType::Premium,
        }
    }

    /// Obtener todos los planes disponibles
    pub fn all_plans() -> Vec<PlanInfo> {
        vec![
            PlanType::Basic.get_plan_info(),
            PlanType::Premium.get_plan_info(),
            PlanType::Enterprise.get_plan_info(),
        ]
    }
}
