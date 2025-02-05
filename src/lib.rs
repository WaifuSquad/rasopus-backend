use config::{database::DatabaseConfig, rasopus::RasopusConfig, rocket::RocketConfig};
use model::{
    user::{
        DbUser,
        Role::{self, System},
        User,
    },
    DbEntity,
};
use orion::pwhash::{hash_password, Password};
use rocket::Rocket;
use rocket_okapi::swagger_ui::*;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use uuid::Uuid;

pub mod config;
pub mod controller;
pub mod database;
pub mod macros;
pub mod model;

pub fn build_rocket(
    rocket_config: RocketConfig,
    managed_data: Vec<Box<dyn Send + Sync + 'static>>,
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

    #[error("Rocket failed: {0}")]
    Rocket(#[from] rocket::Error),
}

pub async fn run(rasopus_config: RasopusConfig) -> Result<(), RuntimeError> {
    println!("Connecting to database");
    let database_config = DatabaseConfig::from(&rasopus_config);
    let database_pool = PgPoolOptions::new()
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

    match DbUser::exists_any_by_role(System, &database_pool).await {
        Ok(true) => println!("A system user exists"),
        Ok(false) => {
            eprintln!("No system user found");
        }
        Err(e) => {
            eprintln!("Failed to check for system user: {}", e);
            return Err(RuntimeError::DatabaseConnect(e));
        }
    }

    let uuid = Uuid::parse_str("2bbc3970-22b6-4bcb-81e9-d38703c790e4").unwrap();

    let exists = DbUser::exists(&uuid, &database_pool).await.unwrap();
    if !exists {
        println!("User does not exist, creating");
        println!("Hashing password...");
        let password = Password::from_slice(b"hi").unwrap();
        let password_hash = hash_password(&password, 3, 1 << 17).unwrap();
        println!("Persisting user");
        let mut user = User::new("Test", Role::System, password_hash);
        user.uuid = uuid;
        let db_user = DbUser::from(user);
        db_user.create(&database_pool).await.unwrap();
    } else {
        println!("User exists");
    }

    let db_user = DbUser::load(&uuid, &database_pool).await.unwrap();
    println!("Loaded user: {:?}", db_user);

    let user = User::try_from(db_user).unwrap();
    println!("User: {:?}", user);

    println!("Building Rocket with Rasopus configuration");
    let rocket_config = RocketConfig::from(&rasopus_config);
    let rocket = build_rocket(rocket_config, vec![Box::new(database_pool)]);

    println!("Launching Rocket");
    let result = rocket.launch().await;

    if let Err(e) = result {
        eprintln!("Rocket had a runtime error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
