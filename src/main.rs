use anyhow::Result;
use rasopus::{
    build_rocket,
    config::{database::DatabaseConfig, rocket_overrides::RocketOverrides},
    parse_env_config, APP_NAME, APP_VERSION,
};

#[rocket::main]
async fn main() -> Result<()> {
    println!("Starting {} v{}", APP_NAME, APP_VERSION);

    println!("Loading Rasopus environment variables");
    let environment_config = match parse_env_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load Rasopus environment variables: {}", e);
            return Err(e.into());
        }
    };

    println!("Connecting to database");
    let database_config = DatabaseConfig::from(&environment_config);

    println!("Building Rocket with Rasopus configuration");
    let rocket_overrides = RocketOverrides::from(&environment_config);
    let rocket = build_rocket(rocket_overrides);

    println!("Starting Rocket");
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
