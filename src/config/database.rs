use rocket::serde::{Deserialize, Serialize};

use super::environment::EnvironmentConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

impl From<EnvironmentConfig> for DatabaseConfig {
    fn from(environment_config: EnvironmentConfig) -> Self {
        Self {
            host: environment_config.database_host,
            port: environment_config.database_port,
            user: environment_config.database_user,
            password: environment_config.database_password,
            database: environment_config.database_database,
            pool_size: environment_config.database_pool_size,
        }
    }
}

impl From<&EnvironmentConfig> for DatabaseConfig {
    fn from(environment_config: &EnvironmentConfig) -> Self {
        Self {
            host: environment_config.database_host.clone(),
            port: environment_config.database_port,
            user: environment_config.database_user.clone(),
            password: environment_config.database_password.clone(),
            database: environment_config.database_database.clone(),
            pool_size: environment_config.database_pool_size,
        }
    }
}
