use rocket::{
    http::Status,
    response::{status, Responder},
    serde::{
        json::{serde_json, Json},
        Deserialize, Serialize,
    },
    Request,
};
use rocket_okapi::JsonSchema;
use thiserror::Error;

use crate::impl_okapi_json_responder;

/// A response containing information about the backend's setup status.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub struct ResponseSetupGet {
    /// Whether the backend needs to be set up.
    pub needs_setup: bool,
}

impl<'r> Responder<'r, 'static> for ResponseSetupGet {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

impl_okapi_json_responder!(ResponseSetupGet, {
    "200" => {
        description: "The backend's setup status was successfully retrieved",
        example: serde_json::json!(ResponseSetupGet { needs_setup: true }),
    }
});

/// An enum representing possible errors that can occur when checking the backend's setup status.
#[derive(Debug, Error, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub enum ResponseSetupGetError {
    /// A database error occurred.
    #[error("Database error: {0}")]
    Database(String),
}

impl From<sqlx::Error> for ResponseSetupGetError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error.to_string())
    }
}

impl<'r> Responder<'r, 'static> for ResponseSetupGetError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = match self {
            Self::Database(_) => 500,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(ResponseSetupGetError, {
    "500" => {
        description: "The backend's setup status could not be retrieved.",
        example: serde_json::json!(ResponseSetupGetError::Database("Connection error".to_string())),
    },
});
