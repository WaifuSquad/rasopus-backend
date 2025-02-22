use std::fmt::Debug;

use crate::model::DbEntity;

pub mod user;

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
