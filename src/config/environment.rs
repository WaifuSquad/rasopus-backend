use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EnvironmentConfig {
    pub address: Option<String>,
    pub port: Option<u16>,

    pub database_host: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_password: String,
    pub database_database: String,
    pub database_pool_size: u32,
}
