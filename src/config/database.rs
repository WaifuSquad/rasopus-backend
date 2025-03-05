use rocket::serde::{Deserialize, Serialize};

use super::rasopus::RasopusConfig;

const DEFAULT_DATABASE_POOL_SIZE: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub pool_size: u32,
}

impl DatabaseConfig {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

impl From<RasopusConfig> for DatabaseConfig {
    fn from(environment_config: RasopusConfig) -> Self {
        Self {
            user: environment_config.database_user,
            password: environment_config.database_password,
            host: environment_config.database_host,
            port: environment_config.database_port,
            database: environment_config.database_database,
            pool_size: environment_config
                .database_pool_size
                .unwrap_or(DEFAULT_DATABASE_POOL_SIZE),
        }
    }
}

impl From<&RasopusConfig> for DatabaseConfig {
    fn from(environment_config: &RasopusConfig) -> Self {
        Self {
            user: environment_config.database_user.clone(),
            password: environment_config.database_password.clone(),
            host: environment_config.database_host.clone(),
            port: environment_config.database_port,
            database: environment_config.database_database.clone(),
            pool_size: environment_config
                .database_pool_size
                .unwrap_or(DEFAULT_DATABASE_POOL_SIZE),
        }
    }
}
