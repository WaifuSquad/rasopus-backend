use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use thiserror::Error;

/// A response containing information about the backend's setup status.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub struct SetupGetResponse {
    /// Whether the backend needs to be set up.
    pub needs_setup: bool,
}

/// An enum representing possible errors that can occur while checking the backend's setup status.
#[derive(Debug, Error, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub enum SetupGetErrorResponse {
    /// An error occurred while interacting with the database.
    #[error("An error occurred while interacting with the database: {0}")]
    Database(String),
}

impl From<sqlx::Error> for SetupGetErrorResponse {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error.to_string())
    }
}
