use sqlx::{query, Pool, Postgres};
use crate::{Verification, Error};

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

pub async fn create_verification_code(executor: &Executor, verification: &Verification) -> Result<()> {
    query(r#"
    INSERT INTO verification_codes (id, user_id, code, created_at)
    VALUES ($1, $2, $3, $4);"#)
    .bind(&verification.id)
    .bind(&verification.user_id)
    .bind(&verification.code)
    .bind(&verification.created_at)
    .execute(executor)
    .await?;
    Ok(())
}
