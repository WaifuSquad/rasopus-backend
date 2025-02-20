use rocket::{
    get,
    serde::{Deserialize, Serialize, json::Json},
};
use rocket_okapi::{JsonSchema, openapi};

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
