use config::{database::DatabaseConfig, rasopus::RasopusConfig, rocket_overrides::RocketOverrides};
use lum_config::{EnvHandler, EnvironmentConfigParseError};
use rocket::Rocket;
use rocket_okapi::swagger_ui::*;
use sqlx::any::AnyPoolOptions;
use thiserror::Error;

pub mod config;
pub mod controller;
pub mod database;
pub mod macros;

pub fn parse_config_from_env<IntoString: Into<String>>(
    app_name: IntoString,
) -> Result<RasopusConfig, EnvironmentConfigParseError> {
    EnvHandler::new(app_name).load_config()
}

pub fn build_rocket(
    rocket_overrides: RocketOverrides,
    managed_data: Vec<Box<dyn Send + Sync + 'static>>,
) -> Rocket<rocket::Build> {
    let mut figment = rocket::Config::figment();
    figment = rocket_overrides.apply(figment);

    let mut rocket = rocket::custom(figment)
        .mount("/", controller::openapi_get_routes())
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),
                deep_linking: true,
                ..Default::default()
            }),
        );

    for data in managed_data {
        rocket = rocket.manage(data);
    }

    rocket
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to connect to database: {0}")]
    DatabaseConnect(#[from] sqlx::Error),

    #[error("Failed to check database migrations: {0}")]
    CheckMigration(#[from] database::CheckMigrationError),

    #[error("Failed to run database migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("Failed to start Rocket: {0}")]
    Rocket(#[from] rocket::Error),
}

pub async fn run(rasopus_config: RasopusConfig) -> Result<(), RuntimeError> {
    println!("Initializing database drivers");
    sqlx::any::install_default_drivers();

    println!("Connecting to database");
    let database_config = DatabaseConfig::from(&rasopus_config);
    let database_pool = AnyPoolOptions::new()
        .max_connections(database_config.pool_size)
        .connect(&database_config.to_connection_string())
        .await?;

    println!("Checking database migrations");
    let migrator = sqlx::migrate!("./migrations");
    let needs_migration = database::needs_migration(&database_pool, &migrator).await?;
    if needs_migration {
        println!("Applying missing database migrations");
        migrator.run(&database_pool).await?;
        println!("Database migrations applied");
    } else {
        println!("Database is up to date");
    }

    println!("Building Rocket with Rasopus configuration");
    let rocket_overrides = RocketOverrides::from(&rasopus_config);
    let rocket = build_rocket(rocket_overrides, vec![Box::new(database_pool)]);

    println!("Starting Rocket");
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
