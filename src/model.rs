use std::fmt::Debug;

use rocket::async_trait;
use sqlx::{Pool, Postgres};

pub mod entity;
pub mod payload;

#[async_trait]
pub trait DbEntity: Sized + Send {
    type Identifier: Clone + Send + 'static;
    type ExistsError: Debug;
    type CreateError: Debug;
    type LoadError: Debug;
    type UpdateError: Debug;
    type PersistError: Debug
        + From<Self::ExistsError>
        + From<Self::CreateError>
        + From<Self::UpdateError>;

    fn main_table_name() -> &'static str;
    fn get_identifier(&self) -> &Self::Identifier;

    async fn exists(
        identifier: &Self::Identifier,
        database_pool: &Pool<Postgres>,
    ) -> Result<bool, Self::ExistsError>;

    async fn create(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::CreateError>;

    async fn load(
        identifier: &Self::Identifier,
        database_pool: &Pool<Postgres>,
    ) -> Result<Self, Self::LoadError>;

    async fn update(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::UpdateError>;

    async fn persist(&self, database_pool: &Pool<Postgres>) -> Result<(), Self::PersistError> {
        let exists = Self::exists(&self.get_identifier().clone(), database_pool).await?;
        if exists {
            self.update(database_pool).await?;
        } else {
            self.create(database_pool).await?;
        }

        Ok(())
    }
}

pub trait DbEntityAdapter<T>: Sized
where
    Self: TryFrom<T>,
    <Self as TryFrom<T>>::Error: Debug,
    T: DbEntity + TryFrom<Self>,
    <T as TryFrom<Self>>::Error: Debug,
{
}

pub trait DbEntityReference<T>: Sized
where
    for<'t> Self: TryFrom<&'t T>,
    for<'t> <Self as TryFrom<&'t T>>::Error: Debug,
    for<'s> T: DbEntity + TryFrom<&'s Self>,
    for<'s> <T as TryFrom<&'s Self>>::Error: Debug,
{
}
