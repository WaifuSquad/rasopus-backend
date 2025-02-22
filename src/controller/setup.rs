use rocket::{
    Request, State, get,
    http::Status,
    post,
    response::{Responder, status},
    serde::json::{Json, serde_json},
};
use rocket_okapi::openapi;
use sqlx::{Pool, Postgres};

use crate::{
    impl_okapi_json_responder,
    model::{
        entity::user::Role,
        payload::setup::{
            SetupGetErrorResponse, SetupGetResponse, SetupPostErrorResponse, SetupPostPayload,
            SetupPostResponse,
        },
    },
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
            Self::SetupServiceCheck(_) => Status::InternalServerError.code,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupGetErrorResponse, {
    "500" => {
        description: "The backend's setup status could not be retrieved.",
        example: serde_json::json!(SetupGetErrorResponse::SetupServiceCheck("The user service returned an error while checking if a system user exists: Database error: pool timed out while waiting for an open connection".to_string())),
    },
});

/// Check whether the backend needs to be set up.
#[openapi]
#[get("/setup")]
pub async fn setup_get(
    user_service: &State<UserService>,
    setup_service: &State<SetupService>,
    database_pool: &State<Pool<Postgres>>,
) -> Result<SetupGetResponse, SetupGetErrorResponse> {
    let needs_setup = setup_service
        .needs_setup(user_service, database_pool)
        .await?;

    Ok(SetupGetResponse { needs_setup })
}

impl<'r> Responder<'r, 'static> for SetupPostResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupPostResponse, {
    "200" => {
        description: "The backend was successfully set up",
        example: serde_json::json!(SetupPostResponse {}),
    }
});

impl<'r> Responder<'r, 'static> for SetupPostErrorResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = match self {
            Self::SetupServiceCheck(_) => Status::InternalServerError.code,
            Self::AlreadySetup => Status::Conflict.code,
            Self::UserServiceGenerate(_) => Status::InternalServerError.code,
            Self::UserServiceCreate(_) => Status::InternalServerError.code,
        };

        status::Custom(Status::new(status_code), Json(self)).respond_to(request)
    }
}

impl_okapi_json_responder!(SetupPostErrorResponse, {
    "400" => {
        description: "The request was malformed.",
        example: serde_json::json!({
            "error": {
              "code": 400,
              "reason": "Bad Request",
              "description": "The request could not be understood by the server due to malformed syntax."
            }
          }),
    },
    "409" => {
        description: "The backend is already set up.",
        example: serde_json::json!(SetupPostErrorResponse::AlreadySetup),
    },
    "422" => {
        description: "The request contained invalid data.",
        example: serde_json::json!({
            "error": {
              "code": 422,
              "reason": "Unprocessable Entity",
              "description": "The request was well-formed but was unable to be followed due to semantic errors."
            }
          }),
    },
    "500" => {
        description: "The backend could not be set up.",
        example: serde_json::json!(SetupPostErrorResponse::UserServiceCreate("The user service returned an error while creating the user in the database: Database error: pool timed out while waiting for an open connection".to_string())),
    },
});

/// Set up the backend.
#[openapi]
#[post("/setup", data = "<payload>")]
pub async fn setup_post(
    payload: Json<SetupPostPayload>,
    setup_service: &State<SetupService>,
    user_service: &State<UserService>,
    database_pool: &State<Pool<Postgres>>,
) -> Result<SetupPostResponse, SetupPostErrorResponse> {
    let needs_setup = setup_service
        .needs_setup(user_service, database_pool)
        .await?;

    if !needs_setup {
        return Err(SetupPostErrorResponse::AlreadySetup);
    }

    let payload = payload.into_inner();
    let (username, password) = (payload.username, payload.password);
    let user = user_service
        .generate(username, &password, Role::System)
        .await?;

    user_service.create(user, database_pool).await?;

    Ok(SetupPostResponse {})
}
