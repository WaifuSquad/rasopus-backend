use rasopus::build_rocket;
use rocket::{launch, Rocket};

#[launch]
fn rocket() -> Rocket<rocket::Build> {
    build_rocket()
}
