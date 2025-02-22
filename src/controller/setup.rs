use rocket::{
    Request, State, get,
    http::Status,
    response::{Responder, status},
    serde::json::{Json, serde_json},
};
use rocket_okapi::openapi;
use sqlx::{Pool, Postgres};

use crate::{
    impl_okapi_json_responder,
    model::payload::setup::{SetupGetErrorResponse, SetupGetResponse},
    service::{SetupService, UserService},
};

impl<'r> Responder<'r, 'static> for SetupGetResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupGetResponse, {
    "200" => {
        description: "The backend's setup status was successfully retrieved",
        example: serde_json::json!(SetupGetResponse { needs_setup: false }),
    }
});

impl<'r> Responder<'r, 'static> for SetupGetErrorResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = match self {
            Self::SetupCheckError(_) => Status::InternalServerError.code,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupGetErrorResponse, {
    "500" => {
        description: "The backend's setup status could not be retrieved.",
        example: serde_json::json!(SetupGetErrorResponse::SetupCheckError("The user service returned an error while checking if a system user exists: Database error: pool timed out while waiting for an open connection".to_string())),
    },
});

/// Check whether the backend needs to be set up.
#[openapi]
#[get("/setup")]
pub async fn setup(
    user_service: &State<UserService>,
    setup_service: &State<SetupService>,
    database_pool: &State<Pool<Postgres>>,
) -> Result<SetupGetResponse, SetupGetErrorResponse> {
    let needs_setup = setup_service
        .needs_setup(user_service, database_pool)
        .await?;

    Ok(SetupGetResponse { needs_setup })
}
