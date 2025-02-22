use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::model::entity::user::Role;

use super::user::{self, UserService};

#[derive(Debug, Error)]
pub enum SetupCheckError {
    #[error("The user service returned an error while checking if a system user exists: {0}")]
    UserServiceExistsError(#[from] user::ExistsError),
}

#[derive(Debug, Default)]
pub struct SetupService;

impl SetupService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn needs_setup(
        &self,
        user_service: &UserService,
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, SetupCheckError> {
        let system_user_exists = user_service
            .exists_any_user_by_role(Role::System, database_pool)
            .await?;

        let needs_setup = !system_user_exists;
        Ok(needs_setup)
    }
}
