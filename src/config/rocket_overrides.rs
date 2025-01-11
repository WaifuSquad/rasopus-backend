use lum_config::MergeFrom;
use rocket::serde::{Deserialize, Serialize};

use super::env::EnvConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", default)]
pub struct RocketOverrides {
    pub address: String,
    pub port: u16,
}

impl Default for RocketOverrides {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 8000,
        }
    }
}

impl MergeFrom<EnvConfig> for RocketOverrides {
    fn merge_from(self, env_config: EnvConfig) -> Self {
        let default = RocketOverrides::default();

        Self {
            address: env_config.address.unwrap_or(default.address),
            port: env_config.port.unwrap_or(default.port),
        }
    }
}
