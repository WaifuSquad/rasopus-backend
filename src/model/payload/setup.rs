use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use thiserror::Error;

use crate::service;

// ### GET ###

/// A response containing information about the backend's setup status.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub struct SetupGetResponse {
    /// Whether the backend needs to be set up.
    pub needs_setup: bool,
}

/// An error response containing one of the possible errors that can occur while checking the backend's setup status.
#[derive(Debug, Error, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub enum SetupGetErrorResponse {
    /// The setup service returned an error while checking whether the backend needs to be set up
    #[error(
        "The setup service returned an error while checking whether the backend needs to be set up: {0}"
    )]
    SetupServiceCheck(String),
}

impl From<service::setup::SetupCheckError> for SetupGetErrorResponse {
    fn from(error: service::setup::SetupCheckError) -> Self {
        Self::SetupServiceCheck(error.to_string())
    }
}

// ### POST ###

/// The payload to set up the backend.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub struct SetupPostPayload {
    // The username of the initial system user to create.
    pub username: String,

    // The password of the initial system user to create.
    pub password: String,
}

/// An empty success response, indicating that the backend has been set up successfully.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub struct SetupPostResponse {}

/// An error response containing one of the possible errors that can occur while setting up the backend.
#[derive(Debug, Error, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(crate = "okapi::schemars")]
pub enum SetupPostErrorResponse {
    /// The setup service returned an error while checking if the backend needs to be set up
    #[error(
        "The setup service returned an error while checking if the backend needs to be set up: {0}"
    )]
    SetupServiceCheck(String),

    /// The backend is already set up
    #[error("The backend is already set up")]
    AlreadySetup,

    /// The user service returned an error while generating the system user
    #[error("The user service returned an error while generating the system user: {0}")]
    UserServiceGenerate(String),

    /// The user service returned an error while creating the system user inside the database
    #[error(
        "The user service returned an error while creating the system user inside the database: {0}"
    )]
    UserServiceCreate(String),
}

impl From<service::setup::SetupCheckError> for SetupPostErrorResponse {
    fn from(error: service::setup::SetupCheckError) -> Self {
        Self::SetupServiceCheck(error.to_string())
    }
}

impl From<service::user::GenerateError> for SetupPostErrorResponse {
    fn from(error: service::user::GenerateError) -> Self {
        Self::UserServiceGenerate(error.to_string())
    }
}

impl From<service::user::CreateError> for SetupPostErrorResponse {
    fn from(error: service::user::CreateError) -> Self {
        Self::UserServiceCreate(error.to_string())
    }
}
