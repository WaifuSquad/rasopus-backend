use sqlx::{Pool, Postgres};

use crate::model::{
    entity::user::{DbUser, Role},
    DbEntity,
};

#[derive(Debug, Default)]
pub struct SetupService;

impl SetupService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn needs_setup(&self, database_pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
        let system_user_exists = check_database_for_system_user(database_pool).await?;
        let needs_setup = !system_user_exists;

        Ok(needs_setup)
    }
}

async fn check_database_for_system_user(
    database_pool: &Pool<Postgres>,
) -> Result<bool, sqlx::Error> {
    let query = format!(
        "SELECT * FROM {} WHERE role = $1 LIMIT 1",
        DbUser::main_table_name()
    );

    let result = sqlx::query(&query)
        .bind::<i16>(Role::System.into())
        .fetch_optional(database_pool)
        .await?;

    Ok(result.is_some())
}
