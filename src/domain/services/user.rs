use sqlx::{Postgres, Pool, query_as, query, FromRow, Error};
use super::{User, Id, EmailAddress};

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn create_user(executor: &Executor, user: User) -> Result<User> {
    user_by_email_does_not_exist(executor, &user.email).await?;
    let created_user = query_as(r#"
    INSERT INTO users 
    (id, email, user_name, first_name, last_name, password, created_at, profile_picture)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING id, email, user_name, first_name, last_name, password, created_at, profile_picture;
    "#,)
    .bind(user.id).bind(user.email).bind(user.user_name).bind(user.first_name).bind(user.last_name).bind(user.password).bind(user.created_at).bind(user.profile_picture)
    .fetch_one(executor).await?;
    Ok(created_user)
}


async fn user_by_email_does_not_exist(executor: &Executor, email: &EmailAddress) -> Result<()> {
    use EmailAddress::*;
    let email = match email{New(address)=>address.to_string(), Verified(address)=>address.to_string()};
    let result = query!(r#"SELECT id FROM users WHERE email->>'email' = $1;"#, email).fetch_one(executor).await;
    match result {
        Ok(record) => Err("user with this email already exists.".into()),
        Err(err) => {
            match err {
                Error::RowNotFound => Ok(()),
                _ => Err(err)?
            }
        }
    }
}