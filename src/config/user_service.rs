use rocket::serde::{Deserialize, Serialize};

use super::rasopus::RasopusConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserServiceConfig {
    pub argon2_iterations: u32,
    pub argon2_memory_mib: u32,
}

// This is way above OWASP's 2025 recommendations, so should be fine :)
// See: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
impl Default for UserServiceConfig {
    fn default() -> Self {
        Self {
            argon2_iterations: 3,
            argon2_memory_mib: 70,
        }
    }
}

impl From<RasopusConfig> for UserServiceConfig {
    fn from(value: RasopusConfig) -> Self {
        let default = Self::default();

        Self {
            argon2_iterations: value.argon2_iterations.unwrap_or(default.argon2_iterations),
            argon2_memory_mib: value.argon2_memory_mib.unwrap_or(default.argon2_memory_mib),
        }
    }
}

impl From<&RasopusConfig> for UserServiceConfig {
    fn from(value: &RasopusConfig) -> Self {
        let default = Self::default();

        Self {
            argon2_iterations: value.argon2_iterations.unwrap_or(default.argon2_iterations),
            argon2_memory_mib: value.argon2_memory_mib.unwrap_or(default.argon2_memory_mib),
        }
    }
}
