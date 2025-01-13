use rocket::serde::{Deserialize, Serialize};

use super::environment::EnvironmentConfig;

pub trait ToConnectionString {
    fn to_connection_string(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum DatabaseType {
    Postgres,
    MySQL,
    Sqlite,
    Memory,
}

impl ToConnectionString for DatabaseType {
    fn to_connection_string(&self) -> String {
        match self {
            DatabaseType::Postgres => "postgres".to_string(),
            DatabaseType::MySQL => "mysql".to_string(),
            DatabaseType::Sqlite => "sqlite".to_string(),
            DatabaseType::Memory => "memory".to_string(),
        }
    }
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

impl ToConnectionString for DatabaseConfig {
    fn to_connection_string(&self) -> String {
        match self.database_type {
            DatabaseType::Memory => "sqlite::memory:".to_string(),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                self.database_type.to_connection_string(),
                self.user,
                self.password,
                self.host,
                self.port,
                self.database
            ),
        }
    }
}

impl ToConnectionString for &DatabaseConfig {
    fn to_connection_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.database_type.to_connection_string(),
            self.user,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }
}

impl From<EnvironmentConfig> for DatabaseConfig {
    fn from(environment_config: EnvironmentConfig) -> Self {
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

impl From<&EnvironmentConfig> for DatabaseConfig {
    fn from(environment_config: &EnvironmentConfig) -> Self {
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
