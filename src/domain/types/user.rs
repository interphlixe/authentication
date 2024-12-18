use serde::{Serialize, Deserialize};
use sqlx::{Encode, Decode, FromRow};
use chrono::{DateTime, offset::Utc, TimeZone};
use super::{Id, EmailAddress};


pub const FIELDS: &'static [&'static str] = &["id", "email", "user_name", "first_name", "last_name", "password", "created_at", "profile_picture"];


#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, FromRow)]
pub struct User {
    #[serde(default)]
    pub id: Id,
    pub email: EmailAddress,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[sqlx(default)]
    pub password: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub profile_picture: Option<String>
}


impl User {
    const LENGTH: usize = FIELDS.len()-1;

    pub fn fields() -> [&'static str; Self::LENGTH] {
        let fields = FIELDS;
        let mut filtered = [""; Self::LENGTH];
        let mut index = 0;
        let mut i = 0;
        while i < fields.len() {
            if fields[i]  != "password" {
                filtered[index] = fields[i];
                index += 1;
            }
            i+=1
        }
        filtered
    }
}