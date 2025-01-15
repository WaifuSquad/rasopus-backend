use anyhow::Result;
use rasopus::{config::rasopus::RasopusConfig, run};

pub static APP_NAME: &str = "Rasopus";
pub static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[rocket::main]
async fn main() -> Result<()> {
    println!("Starting {} v{}", APP_NAME, APP_VERSION);

    println!("Loading Rasopus environment variables");
    let rasopus_config = match RasopusConfig::from_env(APP_NAME) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load Rasopus environment variables: {}", e);
            return Err(e.into());
        }
    };

    let result = run(rasopus_config).await;
    if let Err(e) = result {
        eprintln!("Rasopus had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
