use rocket::get;
use rocket_okapi::openapi;

/// Always works
#[openapi]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
