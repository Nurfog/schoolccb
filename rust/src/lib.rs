pub mod auth;
pub mod features;
pub mod handlers;
pub mod models;
pub mod repository;

pub use features::{FeatureType, PlanType};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub db_connected: Option<bool>,
}
