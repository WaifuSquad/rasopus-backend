pub mod setup;
pub mod user;

pub use setup::SetupService;
pub use user::UserService;

#[derive(Debug, Default)]
pub struct ServiceCollection {
    pub setup: SetupService,
    pub user: UserService,
}
