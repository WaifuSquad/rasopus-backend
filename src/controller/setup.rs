use rocket::{
    get,
    http::Status,
    response::{status, Responder},
    serde::json::{serde_json, Json},
    Request, State,
};
use rocket_okapi::openapi;
use sqlx::{Pool, Postgres};

use crate::{
    impl_okapi_json_responder,
    model::payload::setup::{SetupGetErrorResponse, SetupGetResponse},
    service::setup::SetupService,
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
            Self::SetupCheck(_) => Status::InternalServerError.code,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupGetErrorResponse, {
    "500" => {
        description: "The backend's setup status could not be retrieved.",
        example: serde_json::json!(SetupGetErrorResponse::SetupCheck("An error occurred while checking the backend's setup status: A database error occurred: Lost connection".to_string())),
    },
});

/// Check whether the backend needs to be set up.
#[openapi]
#[get("/setup")]
pub async fn setup(
    database_pool: &State<Pool<Postgres>>,
    setup_service: &State<SetupService>,
) -> Result<SetupGetResponse, SetupGetErrorResponse> {
    let needs_setup = setup_service.needs_setup(database_pool).await?;

    Ok(SetupGetResponse { needs_setup })
}
