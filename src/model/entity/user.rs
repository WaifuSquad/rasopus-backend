use chrono::{DateTime, Utc};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use orion::pwhash::PasswordHash;
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// The possible roles a user can have.
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
    /// The system role, which has the highest level of access.
    System = 0,

    /// The admin role, which has lower access than the system role, but higher access than the user role.
    Admin = 1,

    /// The user role, which has the lowest level of access.
    #[default]
    User = 2,
}

/// The database representation of a user.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct DbUser {
    /// The user's UUID.
    pub uuid: Uuid,

    /// The user's username.
    pub username: String,

    /// The user's role, represented as a 16-bit signed integer.
    pub role: i16,

    /// The user's password hash.
    pub password_hash: String,

    /// The timestamp at which the user was created, represented as a 64-bit signed integer.
    pub created_at: i64,
}

/// A user
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    /// The user's UUID.
    pub uuid: Uuid,

    /// The user's username.
    pub username: String,

    /// The user's role.
    pub role: Role,

    /// The user's password hash.
    pub password_hash: PasswordHash,

    /// The timestamp at which the user was created.
    pub created_at: DateTime<Utc>,
}
