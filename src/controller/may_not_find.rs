use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_okapi::{openapi, JsonSchema};

/// MayNotFind Found Response
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub struct MayNotFindFound {
    pub message: String,
}

/// Get a cool struct, but it may not be found
#[openapi]
#[get("/may_not_find")]
pub fn may_not_find() -> Option<Json<MayNotFindFound>> {
    None
}
