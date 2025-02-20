use rocket::{
    get,
    http::Status,
    response::{Responder, status},
    serde::{
        Deserialize, Serialize,
        json::{Json, serde_json},
    },
};
use rocket_okapi::{JsonSchema, openapi};
use thiserror::Error;

use crate::impl_okapi_json_responder;

/// MayFail Success Response
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub struct MayFailSuccess {
    a: i8,
    b: i16,
    c: i32,
}

/// MayFail Error Response
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema, Error)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub enum MayFailError {
    /// This value occurs on error A
    #[error("A")]
    A,

    /// This value occurs on error B
    #[error("B: {0}")]
    B(i8),

    /// This value occurs on error C
    #[error("C: {val1}, {val2}")]
    C { val1: i16, val2: i32 },
}

impl<'r> Responder<'r, 'static> for MayFailError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status_code = match self {
            MayFailError::A => 700,
            MayFailError::B(_) => 701,
            MayFailError::C { .. } => 702,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(MayFailError, {
    "460" => {
        description: "A happened",
        example: serde_json::json!(MayFailError::A),
    },
    "461" => {
        description: "B happened",
        example: serde_json::json!(MayFailError::B(1)),
    },
    "462" => {
        description: "C happened",
        example: serde_json::json!(MayFailError::C { val1: 1, val2: 2 }),
    },
});

/// Get some data, but it may fail
#[openapi]
#[get("/may_fail")]
pub fn may_fail() -> Result<Json<MayFailSuccess>, MayFailError> {
    Err(MayFailError::C { val1: 1, val2: 2 })
}
