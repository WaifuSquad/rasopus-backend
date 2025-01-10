use rocket::get;
use rocket_okapi::openapi;

/// Returns "Hello, world!".
#[openapi]
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
