use crate::{Verification, Error};
use sqlx::types::Uuid;
use crate::domain::db;
use sqlx::Postgres;
use chrono::Utc;
use sqlx::Pool;
use rand::Rng;
use super::Id;

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

pub async fn generate_verification_code(executor: &Executor, user_id: Id) -> Result<Verification> {
    let code = rand::thread_rng().gen_range(100_000..1_000_000).to_string();
    let verification = Verification {
        id: Uuid::new_v4(),
        user_id,
        code: code.clone(),
        created_at: Utc::now(),
    };
    db::verification::create_verification_code(executor, &verification).await?;
    Ok(verification)
}
