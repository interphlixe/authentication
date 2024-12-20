use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Verification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub created_at: DateTime<Utc>,
}
