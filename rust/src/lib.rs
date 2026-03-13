pub mod auth;
pub mod config;
pub mod email;
pub mod features;
pub mod handlers;
pub mod models;
pub mod repository;

#[cfg(test)]
mod auth_tests;

pub use features::{FeatureType, PlanType};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub db_connected: Option<bool>,
}
