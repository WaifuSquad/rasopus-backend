use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_okapi::{openapi, JsonSchema};

/// A test struct
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub struct TestStruct {
    some_number: i8,
    another_number: i16,
    you_get_it: i32,
}

/// A test enum
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub enum TestEnum {
    FirstOption(i8),
    SecondOption(i16),
    IdkHowThisWillLookInOpenApi(i32, u128),
}

/// Get a test struct
#[openapi]
#[get("/test_struct")]
pub fn test_struct() -> Json<TestStruct> {
    Json(TestStruct {
        some_number: 1,
        another_number: 2,
        you_get_it: 3,
    })
}

/// Get a test enum
#[openapi]
#[get("/test_enum")]
pub fn test_enum() -> Json<TestEnum> {
    Json(TestEnum::FirstOption(1))
}
