use anyhow::Result;
use rasopus::build_rocket;

#[rocket::main]
async fn main() -> Result<()> {
    println!("Starting Rocket");
    let rocket = build_rocket();
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
