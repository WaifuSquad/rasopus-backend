use chrono::DateTime;
use num_enum::TryFromPrimitiveError;
use orion::{errors::UnknownCryptoError, pwhash::PasswordHash};
use rocket::async_trait;
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

use crate::model::{
    entity::user::{DbUser, Role, User},
    DbEntity, DbEntityAdapter, DbEntityReference,
};

#[derive(Debug, Default)]
pub struct UserService;

#[derive(Debug, Error)]
pub enum GenerateError {}

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
    AdaptError(#[from] UnadaptUserError),
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

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn exists(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, sqlx::Error> {
        DbUser::exists(&user.uuid, database_pool).await
    }

    pub async fn create(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), CreateError> {
        if DbUser::exists(&user.uuid, database_pool).await? {
            return Err(CreateError::AlreadyExists);
        }

        let db_user = DbUser::from(&user);
        db_user.create(database_pool).await?;

        Ok(())
    }

    pub async fn load(
        &self,
        identifier: Uuid,
        database_pool: &Pool<Postgres>,
    ) -> Result<Option<User>, LoadError> {
        if !DbUser::exists(&identifier, database_pool).await? {
            return Ok(None);
        }

        let db_user = DbUser::load(&identifier, database_pool).await?;
        let user = User::try_from(&db_user)?;

        Ok(Some(user))
    }

    pub async fn update(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), UpdateError> {
        if !DbUser::exists(&user.uuid, database_pool).await? {
            return Err(UpdateError::NotFound);
        }

        let db_user = DbUser::from(&user);
        db_user.update(database_pool).await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), DeleteError> {
        if !DbUser::exists(&user.uuid, database_pool).await? {
            return Err(DeleteError::NotFound);
        }

        let db_user = DbUser::from(&user);
        db_user.delete(database_pool).await?;

        Ok(())
    }

    pub async fn persist(
        &self,
        user: User,
        database_pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        let db_user = DbUser::from(&user);
        db_user.persist(database_pool).await?;

        Ok(())
    }
}

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
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, Self::ExistsError> {
        let query = format!(
            "SELECT * FROM {} WHERE uuid = $1 LIMIT 1",
            Self::main_table_name()
        );

        let result = sqlx::query(&query)
            .bind(identifier)
            .fetch_optional(database_pool)
            .await?;

        Ok(result.is_some())
    }

    async fn create(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::CreateError> {
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
            .execute(database_pool)
            .await?;

        Ok(())
    }

    async fn load(
        identifier: &Self::Identifier,
        database_pool: &Pool<Postgres>,
    ) -> Result<Self, Self::LoadError> {
        let query = format!(
            "SELECT * FROM {} WHERE uuid = $1 LIMIT 1",
            Self::main_table_name()
        );

        let db_user = sqlx::query_as(&query)
            .bind(identifier)
            .fetch_one(database_pool)
            .await?;

        Ok(db_user)
    }

    async fn update(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::UpdateError> {
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
            .execute(database_pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::DeleteError> {
        let query = format!("DELETE FROM {} WHERE uuid = $1", Self::main_table_name());

        sqlx::query(&query)
            .bind(self.uuid)
            .execute(database_pool)
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

    #[error("The timestamp was out of range: {0}")]
    TimestampParse(i64),
}

impl TryFrom<DbUser> for User {
    type Error = UnadaptUserError;

    fn try_from(db_user: DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: db_user.uuid,
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
            uuid: db_user.uuid,
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
