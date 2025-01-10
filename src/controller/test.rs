use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_okapi::{openapi, JsonSchema};

/// This is how a struct looks like
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub struct TestStruct {
    some_number: i8,
    another_number: i16,
    you_get_it: i32,
}

/// This is how an enum looks like
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub enum TestEnum {
    /// When there is no data
    NoData,

    /// When there is one integer
    OneInteger(i32),

    /// When there is one long
    OneLong(u64),

    /// When there is one boolean
    OneBoolean(bool),

    /// When there are two strings
    TwoStrings(String, String),
}

/// This will always return a `TestStruct`. It can't fail.
#[openapi]
#[get("/test_struct")]
pub fn test_struct() -> Json<TestStruct> {
    Json(TestStruct {
        some_number: 1,
        another_number: 2,
        you_get_it: 3,
    })
}

/// This will always return a `TestEnum`. It can't fail.
#[openapi]
#[get("/test_enum")]
pub fn test_enum() -> Json<TestEnum> {
    Json(TestEnum::OneInteger(1))
}
