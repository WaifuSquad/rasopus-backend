use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EnvConfig {
    pub address: Option<String>,
    pub port: Option<u16>,
}
