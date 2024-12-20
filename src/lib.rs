use rocket::{get, routes, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/num/<num>")]
fn num_u8(num: u8) -> String {
    format!("You entered the u8 number: {}", num)
}

#[get("/num/<num>", rank = 2)]
fn num_u128(num: u128) -> String {
    format!("You entered the u128 number: {}", num)
}

pub fn build_rocket() -> Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, num_u8, num_u128])
}
