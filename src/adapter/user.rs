use chrono::DateTime;
use num_enum::TryFromPrimitiveError;
use orion::{errors::UnknownCryptoError, pwhash::PasswordHash};
use rocket::async_trait;
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

use crate::model::{
    DbEntity,
    entity::user::{DbUser, Role, User},
};

use super::{DbEntityAdapter, DbEntityReference};

#[async_trait]
impl DbEntity for DbUser {
    type Identifier = Uuid;
    type ExistsError = sqlx::Error;
    type CreateError = sqlx::Error;
    type LoadError = sqlx::Error;
    type UpdateError = sqlx::Error;
    type DeleteError = sqlx::Error;
    type PersistError = sqlx::Error;

    fn main_table_name() -> &'static str {
        "users"
    }

    fn get_identifier(&self) -> &Self::Identifier {
        &self.uuid
    }

    async fn exists(
        identifier: &Self::Identifier,
        postgres_pool: &Pool<Postgres>,
    ) -> Result<bool, Self::ExistsError> {
        let query = format!(
            "SELECT * FROM {} WHERE uuid = $1 LIMIT 1",
            Self::main_table_name()
        );

        let result = sqlx::query(&query)
            .bind(identifier)
            .fetch_optional(postgres_pool)
            .await?;

        Ok(result.is_some())
    }

    async fn create(&self, postgres_pool: &Pool<Postgres>) -> Result<(), Self::CreateError> {
        let query = format!(
            "INSERT INTO {} (uuid, username, role, password_hash, created_at) VALUES ($1, $2, $3, $4, $5)",
            Self::main_table_name()
        );

        sqlx::query(&query)
            .bind(self.uuid)
            .bind(&self.username)
            .bind(self.role)
            .bind(&self.password_hash)
            .bind(self.created_at)
            .execute(postgres_pool)
            .await?;

        Ok(())
    }

    async fn load(
        identifier: &Self::Identifier,
        postgres_pool: &Pool<Postgres>,
    ) -> Result<Self, Self::LoadError> {
        let query = format!(
            "SELECT * FROM {} WHERE uuid = $1 LIMIT 1",
            Self::main_table_name()
        );

        let db_user = sqlx::query_as(&query)
            .bind(identifier)
            .fetch_one(postgres_pool)
            .await?;

        Ok(db_user)
    }

    async fn update(&self, postgres_pool: &Pool<Postgres>) -> Result<(), Self::UpdateError> {
        let query = format!(
            "UPDATE {} SET username = $1, role = $2, password_hash = $3, created_at = $4 WHERE uuid = $5",
            Self::main_table_name()
        );

        sqlx::query(&query)
            .bind(&self.username)
            .bind(self.role)
            .bind(&self.password_hash)
            .bind(self.created_at)
            .bind(self.uuid)
            .execute(postgres_pool)
            .await?;

        Ok(())
    }

    async fn delete(
        identifier: &Self::Identifier,
        postgres_pool: &Pool<Postgres>,
    ) -> Result<(), Self::DeleteError> {
        let query = format!("DELETE FROM {} WHERE uuid = $1", Self::main_table_name());

        sqlx::query(&query)
            .bind(identifier)
            .execute(postgres_pool)
            .await?;

        Ok(())
    }
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username,
            role: user.role.into(),
            password_hash: user.password_hash.unprotected_as_encoded().to_string(),
            created_at: user.created_at.timestamp(),
        }
    }
}

impl From<&User> for DbUser {
    fn from(user: &User) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username.clone(),
            role: user.role.into(),
            password_hash: user.password_hash.unprotected_as_encoded().to_string(),
            created_at: user.created_at.timestamp(),
        }
    }
}

#[derive(Debug, Error)]
pub enum UnadaptUserError {
    #[error("Failed to parse UUID: {0}")]
    UuidParse(#[from] uuid::Error),

    #[error("Failed to parse role: {0}")]
    RoleParse(#[from] TryFromPrimitiveError<Role>),

    #[error("Failed to parse password hash: {0}")]
    PasswordHashParse(#[from] UnknownCryptoError),

    #[error("Failed to parse created_at timestamp: The timestamp was out of range: {0}")]
    CreatedAtParse(i64),
}

impl TryFrom<DbUser> for User {
    type Error = UnadaptUserError;

    fn try_from(db_user: DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: db_user.uuid,
            username: db_user.username,
            role: Role::try_from(db_user.role)?,
            password_hash: PasswordHash::from_encoded(&db_user.password_hash)?,
            created_at: DateTime::from_timestamp(db_user.created_at, 0)
                .ok_or(UnadaptUserError::CreatedAtParse(db_user.created_at))?,
        })
    }
}

impl TryFrom<&DbUser> for User {
    type Error = UnadaptUserError;

    fn try_from(db_user: &DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: db_user.uuid,
            username: db_user.username.clone(),
            role: Role::try_from(db_user.role)?,
            password_hash: PasswordHash::from_encoded(&db_user.password_hash)?,
            created_at: DateTime::from_timestamp(db_user.created_at, 0)
                .ok_or(UnadaptUserError::CreatedAtParse(db_user.created_at))?,
        })
    }
}

impl DbEntityAdapter<DbUser> for User {}
impl DbEntityReference<DbUser> for User {}
