use rocket::Rocket;
use rocket_okapi::swagger_ui::*;

pub mod controller;
pub mod macros;

pub fn build_rocket() -> Rocket<rocket::Build> {
    rocket::build()
        .mount("/", controller::openapi_get_routes())
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),
                deep_linking: true,
                ..Default::default()
            }),
        )
}
