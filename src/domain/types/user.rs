use serde::{Serialize, Deserialize};
use sqlx::{Encode, Decode, FromRow};
use super::{Id, EmailAddress};
use chrono::{DateTime, offset::Utc};

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, FromRow)]
pub struct User {
    #[serde(default)]
    pub id: Id,
    pub email: EmailAddress,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub profile_picture: Option<String>
}