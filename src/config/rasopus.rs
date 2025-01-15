use lum_config::{EnvHandler, EnvironmentConfigParseError};
use rocket::serde::{Deserialize, Serialize};

use super::database::DatabaseType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RasopusConfig {
    pub address: Option<String>,
    pub port: Option<u16>,

    pub database_type: DatabaseType,
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_database: String,
    pub database_pool_size: u32,
}

impl RasopusConfig {
    pub fn from_env<IntoString: Into<String>>(
        app_name: IntoString,
    ) -> Result<Self, EnvironmentConfigParseError> {
        EnvHandler::new(app_name).load_config()
    }
}
