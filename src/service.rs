pub mod setup;
pub mod user;

pub use setup::SetupService;
pub use user::UserService;

use crate::config::{rasopus::RasopusConfig, user_service::UserServiceConfig};

#[derive(Debug)]
pub struct ServiceCollection {
    pub setup: SetupService,
    pub user: UserService,
}

impl ServiceCollection {
    pub fn new(config: &RasopusConfig) -> Self {
        let setup_service = SetupService::new();

        let user_service_config = UserServiceConfig::from(config);
        let user_service = UserService::new(user_service_config);

        Self {
            setup: setup_service,
            user: user_service,
        }
    }
}
