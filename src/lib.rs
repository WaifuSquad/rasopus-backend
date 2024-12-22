use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
    Rocket,
};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*, JsonSchema};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub enum TestEnum {
    FirstOption(i8),
    SecondOption(i16),
    IdkHowThisWillLookInOpenApi(i32, u128),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(crate = "okapi::schemars")]
pub struct TestStruct {
    some_number: i8,
    another_number: i16,
    you_get_it: i32,
}

#[openapi]
#[get("/test_enum")]
fn test_enum() -> Json<TestEnum> {
    Json(TestEnum::FirstOption(1))
}

#[openapi]
#[get("/test_struct")]
fn test_struct() -> Json<TestStruct> {
    Json(TestStruct {
        some_number: 1,
        another_number: 2,
        you_get_it: 3,
    })
}

#[openapi()]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn build_rocket() -> Rocket<rocket::Build> {
    rocket::build()
        .mount("/", openapi_get_routes![index, test_enum, test_struct])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),
                deep_linking: true,
                ..Default::default()
            }),
        )
}
