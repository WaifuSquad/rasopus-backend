use rocket::serde::{Deserialize, Serialize};

use super::rasopus::RasopusConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum DatabaseType {
    #[serde(rename = "postgres")]
    Postgres,

    #[serde(rename = "mysql")]
    MySQL,

    #[serde(rename = "sqlite")]
    Sqlite,

    #[serde(rename = "memory")]
    Memory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub pool_size: u32,
}

impl DatabaseConfig {
    pub fn to_connection_string(&self) -> String {
        match self.database_type {
            DatabaseType::Memory => "sqlite::memory:".to_string(),
            _ => {
                let database_type_string = match self.database_type {
                    DatabaseType::Postgres => "postgres".to_string(),
                    DatabaseType::MySQL => "mysql".to_string(),
                    DatabaseType::Sqlite => "sqlite".to_string(),
                    DatabaseType::Memory => "memory".to_string(),
                };

                format!(
                    "{}://{}:{}@{}:{}/{}",
                    database_type_string,
                    self.user,
                    self.password,
                    self.host,
                    self.port,
                    self.database
                )
            }
        }
    }
}

impl From<RasopusConfig> for DatabaseConfig {
    fn from(environment_config: RasopusConfig) -> Self {
        Self {
            database_type: environment_config.database_type,
            user: environment_config.database_user,
            password: environment_config.database_password,
            host: environment_config.database_host,
            port: environment_config.database_port,
            database: environment_config.database_database,
            pool_size: environment_config.database_pool_size,
        }
    }
}

impl From<&RasopusConfig> for DatabaseConfig {
    fn from(environment_config: &RasopusConfig) -> Self {
        Self {
            database_type: environment_config.database_type.clone(),
            user: environment_config.database_user.clone(),
            password: environment_config.database_password.clone(),
            host: environment_config.database_host.clone(),
            port: environment_config.database_port,
            database: environment_config.database_database.clone(),
            pool_size: environment_config.database_pool_size,
        }
    }
}
