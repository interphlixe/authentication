use sqlx::{Postgres, Pool, query_as, query, FromRow, Error as SqlxError};
use super::{User, Id, EmailAddress, Error};

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

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
        Ok(record) => Err(Error::UserWithEmailExists),
        Err(err) => {
            match err {
                SqlxError::RowNotFound => Ok(()),
                _ => Err(err)?
            }
        }
    }
}


pub async fn get_user_by_id(executor: &Executor, id: Id) -> Result<User> {
    let result = query_as("SELECT * FROM users_view WHERE id = $1").bind(id).fetch_one(executor).await;
    match result {
        Ok(user) => Ok(user),
        Err(err) => {
            match err {
                SqlxError::RowNotFound => Err(Error::UserNotFound),
                _ => Err(err)?
            }
        }
    }
}


pub async fn delete_user_by_id(executor: &Executor, id: Id) -> Result<()> {
    // query!("DELETE FROM users WHERE id = $1", &id.bytes()).execute(executor).await?;
    Ok(())
}