use sqlx::{
    error::DatabaseError as SqlxDatabaseError,
    migrate::{Migrate, MigrateError as SqlxMigrateError, Migrator},
    Any, Error as SqlxError, Pool,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckMigrationError {
    #[error("Failed to acquire connection from pool")]
    Sqlx(#[from] SqlxError),

    #[error("Failed to run migrations")]
    SqlxMigrate(#[from] SqlxMigrateError),

    #[error("Failed to execute database query")]
    SqlxDatabase(#[from] Box<dyn SqlxDatabaseError>),
}

pub async fn needs_migration(
    pool: &Pool<Any>,
    migrator: &Migrator,
) -> Result<bool, CheckMigrationError> {
    let all_checksums = migrator
        .iter()
        .map(|migration| migration.checksum.clone())
        .collect::<Vec<_>>();

    let mut connection = pool.acquire().await?;
    let applied_migrations_result = connection.list_applied_migrations().await;
    match applied_migrations_result {
        Err(SqlxMigrateError::Execute(SqlxError::Database(error))) => {
            if error.message().contains("_sqlx_migrations") {
                // Can't find _sqlx_migrations table because no migrations have ever been applied!
                return Ok(true);
            }

            Err(error.into())
        }

        Err(error) => Err(error.into()),

        Ok(applied_migrations) => {
            let applied_checksums = applied_migrations
                .iter()
                .map(|migration| migration.checksum.clone())
                .collect::<Vec<_>>();
            connection.close().await?;

            let not_applied_checksums = all_checksums
                .iter()
                .filter(|checksum| !applied_checksums.contains(checksum))
                .collect::<Vec<_>>();

            Ok(!not_applied_checksums.is_empty())
        }
    }
}
