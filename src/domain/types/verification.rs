use serde::{Serialize, Deserialize};
use sqlx::{types::Uuid, FromRow};
use chrono::{DateTime, Utc};
use super::Id;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Verification {
    pub id: Uuid,
    pub user_id: Id,
    pub code: String,
    pub created_at: DateTime<Utc>,
}
