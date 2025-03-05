use orion::{
    errors::UnknownCryptoError,
    pwhash::{self, Password},
};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    adapter::user::UnadaptUserError,
    config::user_service::UserServiceConfig,
    model::{
        DbEntity,
        entity::user::{DbUser, Role, User},
    },
};

const BYTES_PER_MB: u32 = 1024 * 1024;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("The given password was empty")]
    EmptyPassword,

    #[error("Cryptography error: {0}")]
    Cryptography(#[from] UnknownCryptoError),
}

#[derive(Debug, Error)]
pub enum ExistsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("User already exists")]
    AlreadyExists,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Failed to unadapt user from database user: {0}")]
    Unadapt(#[from] UnadaptUserError),
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("User not found")]
    NotFound,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("User not found")]
    NotFound,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug)]
pub struct UserService {
    config: UserServiceConfig,
}

impl UserService {
    pub fn new(config: UserServiceConfig) -> Self {
        Self { config }
    }

    pub async fn generate(
        &self,
        username: String,
        password: &str,
        role: Role,
    ) -> Result<User, GenerateError> {
        if password.is_empty() {
            return Err(GenerateError::EmptyPassword);
        }

        let mb = self.config.argon2_memory_mib;
        let bytes = mb * BYTES_PER_MB / 1024; // We have to convert back from MiB to KiB because orion expects KiB
        let iterations = self.config.argon2_iterations;

        let password = Password::from_slice(password.as_bytes())?;
        let password_hash = pwhash::hash_password(&password, iterations, bytes)?;

        let uuid = Uuid::new_v4();
        let created_at = chrono::Utc::now();
        let user = User {
            uuid,
            username,
            role,
            password_hash,
            created_at,
        };

        Ok(user)
    }

    pub async fn exists_any_user_by_role(
        &self,
        role: Role,
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, ExistsError> {
        let query = format!(
            "SELECT * FROM {} WHERE role = $1 LIMIT 1",
            DbUser::main_table_name()
        );

        let result = sqlx::query(&query)
            .bind::<i16>(role.into())
            .fetch_optional(database_pool)
            .await?;

        Ok(result.is_some())
    }

    pub async fn exists(
        &self,
        user: &User,
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, ExistsError> {
        let exists = DbUser::exists(&user.uuid, database_pool).await?;

        Ok(exists)
    }

    pub async fn create(
        &self,
        user: &User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), CreateError> {
        if DbUser::exists(&user.uuid, database_pool).await? {
            return Err(CreateError::AlreadyExists);
        }

        let db_user = DbUser::from(user);
        db_user.create(database_pool).await?;

        Ok(())
    }

    pub async fn load(
        &self,
        identifier: &Uuid,
        database_pool: &Pool<Postgres>,
    ) -> Result<Option<User>, LoadError> {
        if !DbUser::exists(identifier, database_pool).await? {
            return Ok(None);
        }

        let db_user = DbUser::load(identifier, database_pool).await?;
        let user = User::try_from(&db_user)?;

        Ok(Some(user))
    }

    pub async fn update(
        &self,
        user: &User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), UpdateError> {
        if !DbUser::exists(&user.uuid, database_pool).await? {
            return Err(UpdateError::NotFound);
        }

        let db_user = DbUser::from(user);
        db_user.update(database_pool).await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        user: &User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), DeleteError> {
        if !DbUser::exists(&user.uuid, database_pool).await? {
            return Err(DeleteError::NotFound);
        }

        DbUser::delete(&user.uuid, database_pool).await?;

        Ok(())
    }

    pub async fn persist(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), PersistError> {
        let db_user = DbUser::from(&user);
        db_user.persist(database_pool).await?;

        Ok(())
    }
}
