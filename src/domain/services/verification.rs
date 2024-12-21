use crate::domain::db::{verification::*, user::verify_user};
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

    // Verify the user's email and return the updated user
    let updated_user = verify_user(executor, &verification.user_id).await?;

    Ok(updated_user)
}
