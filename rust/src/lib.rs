pub mod auth;
pub mod communications_repository;
pub mod config;
pub mod email;
pub mod email_queue;
pub mod features;
pub mod finance_repository;
pub mod handlers;
pub mod models;
pub mod pdf_generator;
pub mod repository;
pub mod security_repository;
pub mod ai_module;

pub use features::{FeatureType, PlanType};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub db_connected: Option<bool>,
}
