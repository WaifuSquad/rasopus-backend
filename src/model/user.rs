use chrono::{DateTime, Utc};
use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};
use orion::{errors::UnknownCryptoError, pwhash::PasswordHash};
use rocket::{
    async_trait,
    serde::{Deserialize, Serialize},
};
use sqlx::{prelude::FromRow, Any, Pool};
use thiserror::Error;
use uuid::Uuid;

use super::{DbEntity, DbEntityAdapter, DbEntityReference};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(i16)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    System = 0,
    Admin = 1,
    #[default]
    User = 2,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct DbUser {
    pub uuid: String,
    pub username: String,
    pub role: i16,
    pub password_hash: String,
    pub created_at: i64,
}

impl DbUser {
    async fn exists_by_role(role: Role, database_pool: &Pool<Any>) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT * FROM $1 WHERE role = $2 LIMIT 1")
            .bind(Self::main_table_name())
            .bind::<i16>(role.into())
            .fetch_one(database_pool)
            .await;

        if let Err(error) = result {
            if matches!(error, sqlx::Error::RowNotFound) {
                return Ok(false);
            } else {
                return Err(error);
            }
        }

        Ok(true)
    }
}

#[async_trait]
impl DbEntity for DbUser {
    type Identifier = String;
    type ExistsError = sqlx::Error;
    type CreateError = sqlx::Error;
    type LoadError = sqlx::Error;
    type UpdateError = sqlx::Error;
    type PersistError = sqlx::Error;

    fn main_table_name() -> &'static str {
        "users"
    }

    fn get_identifier(&self) -> &Self::Identifier {
        &self.uuid
    }

    async fn exists(
        identifier: &Self::Identifier,
        database_pool: &Pool<Any>,
    ) -> Result<bool, Self::ExistsError> {
        let result = sqlx::query("SELECT * FROM $1 WHERE uuid = $2 LIMIT 1")
            .bind(Self::main_table_name())
            .bind(identifier)
            .fetch_one(database_pool)
            .await;

        if let Err(error) = result {
            if matches!(error, sqlx::Error::RowNotFound) {
                return Ok(false);
            } else {
                return Err(error);
            }
        }

        Ok(true)
    }

    async fn create(&self, database_pool: &Pool<Any>) -> Result<(), Self::CreateError> {
        sqlx::query(
            "INSERT INTO $1 (uuid, username, role, password_hash, created_at) VALUES ($2, $3, $4, $5, $6)",
        )
        .bind(Self::main_table_name())
        .bind(&self.uuid)
        .bind(&self.username)
        .bind(self.role)
        .bind(&self.password_hash)
        .bind(self.created_at)
        .execute(database_pool)
        .await?;

        Ok(())
    }

    async fn load(
        identifier: &Self::Identifier,
        database_pool: &Pool<Any>,
    ) -> Result<Self, Self::LoadError> {
        let db_user = sqlx::query_as("SELECT * FROM $1 WHERE uuid = $2 LIMIT 1")
            .bind(Self::main_table_name())
            .bind(identifier)
            .fetch_one(database_pool)
            .await?;

        Ok(db_user)
    }

    async fn update(&self, database_pool: &Pool<Any>) -> Result<(), Self::UpdateError> {
        sqlx::query(
            "UPDATE $1 SET username = $2, role = $3, password_hash = $4, created_at = $5 WHERE uuid = $6",
        )
        .bind(Self::main_table_name())
        .bind(&self.username)
        .bind(self.role)
        .bind(&self.password_hash)
        .bind(self.created_at)
        .bind(&self.uuid)
        .execute(database_pool)
        .await?;

        Ok(())
    }
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        Self {
            uuid: user.uuid.to_string(),
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
            uuid: user.uuid.to_string(),
            username: user.username.clone(),
            role: user.role.into(),
            password_hash: user.password_hash.unprotected_as_encoded().to_string(),
            created_at: user.created_at.timestamp(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub role: Role,
    pub password_hash: PasswordHash,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new<IntoString: Into<String>>(
        username: IntoString,
        role: Role,
        password_hash: PasswordHash,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username: username.into(),
            role,
            password_hash,
            created_at: Utc::now(),
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

    #[error("The timestamp was out of range: {0}")]
    TimestampParse(i64),
}

impl TryFrom<DbUser> for User {
    type Error = UnadaptUserError;

    fn try_from(db_user: DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: Uuid::try_parse(&db_user.uuid)?,
            username: db_user.username,
            role: Role::try_from(db_user.role).unwrap(),
            password_hash: PasswordHash::from_encoded(&db_user.password_hash)?,
            created_at: DateTime::from_timestamp(db_user.created_at, 0)
                .ok_or(UnadaptUserError::TimestampParse(db_user.created_at))?,
        })
    }
}

impl TryFrom<&DbUser> for User {
    type Error = UnadaptUserError;

    fn try_from(db_user: &DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: Uuid::try_parse(&db_user.uuid)?,
            username: db_user.username.clone(),
            role: Role::try_from(db_user.role).unwrap(),
            password_hash: PasswordHash::from_encoded(&db_user.password_hash)?,
            created_at: DateTime::from_timestamp(db_user.created_at, 0)
                .ok_or(UnadaptUserError::TimestampParse(db_user.created_at))?,
        })
    }
}

impl DbEntityAdapter<DbUser> for User {}
impl DbEntityReference<DbUser> for User {}
