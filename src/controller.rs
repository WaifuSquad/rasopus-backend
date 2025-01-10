use rocket::Route;
use rocket_okapi::openapi_get_routes;

pub mod index;
pub mod may_fail;
pub mod may_not_find;
pub mod test;

pub fn openapi_get_routes() -> Vec<Route> {
    openapi_get_routes![
        index::index,
        may_fail::may_fail,
        may_not_find::may_not_find,
        test::test_struct,
        test::test_enum
    ]
}
