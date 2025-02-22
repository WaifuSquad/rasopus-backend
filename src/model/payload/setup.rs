use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use thiserror::Error;

use crate::service::setup::SetupCheckError;

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
    /// The setup service returned an error while checking if the backend needs to be set up
    #[error(
        "The setup service returned an error while checking if the backend needs to be set up: {0}"
    )]
    SetupCheckError(String),
}

impl From<SetupCheckError> for SetupGetErrorResponse {
    fn from(error: SetupCheckError) -> Self {
        Self::SetupCheckError(error.to_string())
    }
}
