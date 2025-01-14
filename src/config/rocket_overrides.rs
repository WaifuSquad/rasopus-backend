use rocket::{
    figment::Figment,
    serde::{Deserialize, Serialize},
};

use super::rasopus::RasopusConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", default)]
pub struct RocketOverrides {
    pub address: Option<String>,
    pub port: Option<u16>,
}

impl RocketOverrides {
    pub fn apply(self, mut rocket_figment: Figment) -> Figment {
        if let Some(address) = &self.address {
            rocket_figment = rocket_figment.merge(("address", address));
        }

        if let Some(port) = &self.port {
            rocket_figment = rocket_figment.merge(("port", port));
        }

        rocket_figment
    }
}

impl From<RasopusConfig> for RocketOverrides {
    fn from(environment_config: RasopusConfig) -> Self {
        Self {
            address: environment_config.address,
            port: environment_config.port,
        }
    }
}

impl From<&RasopusConfig> for RocketOverrides {
    fn from(environment_config: &RasopusConfig) -> Self {
        Self {
            address: environment_config.address.clone(),
            port: environment_config.port,
        }
    }
}
