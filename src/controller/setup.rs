use rocket::{get, State};
use rocket_okapi::openapi;
use sqlx::{Pool, Postgres};

use crate::model::{
    entity::user::{DbUser, Role},
    payload::setup::{ResponseSetupGet, ResponseSetupGetError},
};

/// Check whether the backend needs to be set up.
#[openapi]
#[get("/setup")]
pub async fn setup(
    database_pool: &State<Pool<Postgres>>,
) -> Result<ResponseSetupGet, ResponseSetupGetError> {
    let exists = DbUser::exists_any_by_role(Role::System, database_pool.inner()).await?;

    Ok(ResponseSetupGet {
        needs_setup: !exists,
    })
}
