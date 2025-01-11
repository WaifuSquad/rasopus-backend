use anyhow::Result;
use lum_config::merge;
use rasopus::{
    build_rocket, config::rocket_overrides::RocketOverrides, parse_env_config, APP_NAME,
    APP_VERSION,
};

#[rocket::main]
async fn main() -> Result<()> {
    println!("Starting {} v{}", APP_NAME, APP_VERSION);

    println!("Loading Rasopus environment variables");
    let env_config = match parse_env_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load Rasopus environment variables: {}", e);
            return Err(e.into());
        }
    };

    println!("Building Rocket with Rasopus configuration");
    let rocket_overrides = RocketOverrides::default();
    let rocket_overrides = merge(rocket_overrides, env_config);
    let rocket = build_rocket(rocket_overrides);

    println!("Starting Rocket");
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
