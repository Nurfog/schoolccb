pub mod auth;
pub mod handlers;
pub mod models;
pub mod repository;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub db_connected: Option<bool>,
}
