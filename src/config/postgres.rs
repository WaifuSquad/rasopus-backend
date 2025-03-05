use rocket::serde::{Deserialize, Serialize};

use super::rasopus::RasopusConfig;

const DEFAULT_POSTGRES_POOL_SIZE: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub pool_size: u32,
}

impl PostgresConfig {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

impl From<RasopusConfig> for PostgresConfig {
    fn from(environment_config: RasopusConfig) -> Self {
        Self {
            user: environment_config.postgres_user,
            password: environment_config.postgres_password,
            host: environment_config.postgres_host,
            port: environment_config.postgres_port,
            database: environment_config.postgres_database,
            pool_size: environment_config
                .postgres_pool_size
                .unwrap_or(DEFAULT_POSTGRES_POOL_SIZE),
        }
    }
}

impl From<&RasopusConfig> for PostgresConfig {
    fn from(environment_config: &RasopusConfig) -> Self {
        Self {
            user: environment_config.postgres_user.clone(),
            password: environment_config.postgres_password.clone(),
            host: environment_config.postgres_host.clone(),
            port: environment_config.postgres_port,
            database: environment_config.postgres_database.clone(),
            pool_size: environment_config
                .postgres_pool_size
                .unwrap_or(DEFAULT_POSTGRES_POOL_SIZE),
        }
    }
}
