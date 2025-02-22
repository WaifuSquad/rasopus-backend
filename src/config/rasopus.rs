use lum_config::{EnvHandler, EnvironmentConfigParseError};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RasopusConfig {
    //We have to duplicate the fields here as serde's flatten doesn't support prefixing
    //I also tried serde_with but it doesn't seem to work with reading from environment variables
    //See: https://github.com/serde-rs/serde/issues/2071

    //Rocket
    pub address: Option<String>,
    pub port: Option<u16>,

    //Database
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_database: String,
    pub database_pool_size: u32,

    //UserService
    pub argon2_iterations: Option<u32>,
    pub argon2_memory_mib: Option<u32>,
}

impl RasopusConfig {
    pub fn from_env<IntoString: Into<String>>(
        app_name: IntoString,
    ) -> Result<Self, EnvironmentConfigParseError> {
        EnvHandler::new(app_name).load_config()
    }
}
