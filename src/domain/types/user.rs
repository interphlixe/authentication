use serde::{Serialize, Deserialize};
use sqlx::{Encode, Decode, FromRow};
// use chrono::{DateTime, offset::Utc};
use super::{Id, EmailAddress};

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, FromRow)]
pub struct User {
    #[serde(default)]
    pub id: Id,
    pub email: EmailAddress,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    #[serde(default)]
    // pub created_at: DateTime<Utc>,
    pub profile_picture: Option<String>
}