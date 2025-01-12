use config::{environment::EnvironmentConfig, rocket_overrides::RocketOverrides};
use lum_config::{EnvHandler, EnvironmentConfigParseError};
use rocket::Rocket;
use rocket_okapi::swagger_ui::*;

pub mod config;
pub mod controller;
pub mod macros;

pub static APP_NAME: &str = "Rasopus";
pub static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_env_config() -> Result<EnvironmentConfig, EnvironmentConfigParseError> {
    EnvHandler::new(APP_NAME).load_config()
}

pub fn build_rocket(rocket_overrides: RocketOverrides) -> Rocket<rocket::Build> {
    let mut figment = rocket::Config::figment();
    figment = rocket_overrides.apply(figment);

    rocket::custom(figment)
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
