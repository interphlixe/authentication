use sqlx::{query, query_as, Pool, Postgres, types::Uuid};
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


pub async fn get_verification_by_id(executor: &Executor, id: &Uuid) -> Result<Verification> {
    let sql = "SELECT * FROM verification_codes WHERE id = $1";
    let result = query_as::<_, Verification>(sql)
        .bind(id)
        .fetch_one(executor)
        .await;

    match result {
        Ok(verification) => Ok(verification),
        Err(sqlx::Error::RowNotFound) => Err(Error::Custom(StatusCode::NOT_FOUND, "Verification code not found".into())),
        Err(err) => Err(Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error".into())),
    }
}


pub async fn get_latest_verification_by_user_id(executor: &Executor, user_id: &Id) -> Result<Verification> {
    let sql = r#"
        SELECT * FROM verification_codes
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 1;
    "#;
    let result = query_as::<_, Verification>(sql)
        .bind(user_id)
        .fetch_one(executor)
        .await;

    match result {
        Ok(verification) => Ok(verification),
        Err(sqlx::Error::RowNotFound) => Err(Error::Custom(StatusCode::NOT_FOUND, "No verification code found for the user".into())),
        Err(err) => Err(Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error".into())),
    }
}



pub async fn delete_verification_by_id(executor: &Executor, id: &Uuid) -> Result<()> {
    let sql = "DELETE FROM verification_codes WHERE id = $1";
    query(sql)
        .bind(id)
        .execute(executor)
        .await?;

    Ok(())
}