use crate::domain::db::{verification::*, user::verify_user};
use actix_web::http::StatusCode;
use sqlx::{Postgres, types::Uuid};
use crate::{Verification, Error};
use super::{Id, User};
use chrono::Utc;
use sqlx::Pool;
use rand::Rng;

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
    create_verification_code(executor, &verification).await?;
    Ok(verification)
}


pub async fn verify_magic_link(executor: &Executor, verification_id: &Uuid) -> Result<User> {
    // Retrieve the verification information by ID
    let verification = get_verification_by_id(executor, verification_id).await?;

    // Delete the verification code
    delete_verification_by_id(executor, &verification.id).await?;

    // Verify the user's email and return the updated user
    let updated_user = verify_user(executor, &verification.user_id).await?;

    Ok(updated_user)
}


pub async fn verify_code_and_update_user(executor: &Executor, user_id: Id, code: &str) -> Result<User> {
    // Retrieve the latest verification code for the user
    let verification = get_latest_verification_by_user_id(executor, &user_id).await?;

    // Check if the code matches
    if verification.code != code {
        return Err(Error::Custom(StatusCode::BAD_REQUEST, "Invalid verification code".into()));
    }

    // Delete the verification code
    delete_verification_by_id(executor, &verification.id).await?;

    // Verify the user's email and return the updated user
    let updated_user = verify_user(executor, &user_id).await?;

    Ok(updated_user)
}