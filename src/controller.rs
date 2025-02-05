use rocket::Route;
use rocket_okapi::openapi_get_routes;

pub mod setup;

pub fn openapi_get_routes() -> Vec<Route> {
    openapi_get_routes![setup::setup]
}
