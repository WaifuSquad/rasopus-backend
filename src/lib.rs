use config::{postgres::PostgresConfig, rasopus::RasopusConfig, rocket::RocketConfig};
use rocket::Rocket;
use rocket_okapi::swagger_ui::*;
use service::ServiceCollection;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use thiserror::Error;

pub mod adapter;
pub mod config;
pub mod controller;
pub mod database;
pub mod macros;
pub mod model;
pub mod service;

pub fn build_rocket(
    rocket_config: RocketConfig,
    postgres_pool: Pool<Postgres>,
    service_collection: ServiceCollection,
) -> Rocket<rocket::Build> {
    let mut figment = rocket::Config::figment();
    figment = rocket_config.apply(figment);

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

    rocket = rocket.manage(postgres_pool);
    rocket = rocket.manage(service_collection.user);
    rocket = rocket.manage(service_collection.setup);

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

    #[error("Rocket failed: {0}")]
    Rocket(#[from] rocket::Error),
}

pub async fn run(rasopus_config: RasopusConfig) -> Result<(), RuntimeError> {
    println!("Connecting to database");
    let postgres_config = PostgresConfig::from(&rasopus_config);
    let postgres_pool = PgPoolOptions::new()
        .max_connections(postgres_config.pool_size)
        .connect(&postgres_config.to_connection_string())
        .await?;

    println!("Checking database migrations");
    let migrator = sqlx::migrate!("./migrations");
    let needs_migration = database::needs_migration(&postgres_pool, &migrator).await?;
    if needs_migration {
        println!("Applying missing database migrations");
        migrator.run(&postgres_pool).await?;
        println!("Database migrations applied");
    } else {
        println!("Database is up to date");
    }

    println!("Initializing services");
    let service_collection = ServiceCollection::new(&rasopus_config);

    println!("Building Rocket with Rasopus configuration");
    let rocket_config = RocketConfig::from(&rasopus_config);
    let rocket = build_rocket(rocket_config, postgres_pool, service_collection);

    println!("Launching Rocket");
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
