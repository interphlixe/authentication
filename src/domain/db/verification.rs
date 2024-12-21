use sqlx::{query, query_as, Pool, Postgres};
use crate::{Verification, Error};
use actix_web::http::StatusCode;
use super::Id;

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


pub async fn get_verification_by_id(executor: &Executor, id: &Id) -> Result<Verification> {
    let sql = "SELECT id, user_id, code, created_at FROM verification_codes WHERE id = $1";
    let result = query_as::<_, Verification>(sql)
        .bind(id)
        .fetch_one(executor)
        .await;

    match result {
        Ok(verification) => Ok(verification),
        Err(sqlx::Error::RowNotFound) => Err(Error::Custom(StatusCode::NOT_FOUND, "Verification code not found".into())),
        Err(err) => Err(Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, err.into())),
    }
}
