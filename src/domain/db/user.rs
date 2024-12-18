use actix_web::http::StatusCode;
use sqlx::{query, query_as, Error as SqlxError, Execute, Pool, Postgres};
use std::collections::HashMap;
use super::Value;
use super::*;

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

///This function inserts a new user into the database.
/// This function first makes sure that a usert does not exist before it creates a new one.
pub async fn create_user(executor: &Executor, user: &User) -> Result<()> {
    user_by_email_does_not_exist(executor, &user.email).await?;
    query(r#"
    INSERT INTO users 
    (id, email, user_name, first_name, last_name, password, created_at, profile_picture)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8);"#,)
    .bind(&user.id).bind(&user.email).bind(&user.user_name).bind(&user.first_name).bind(&user.last_name).bind(&user.password).bind(user.created_at).bind(&user.profile_picture)
    .execute(executor).await?;
    Ok(())
}


///This function checks if a user with the provided email is in the database.
/// If a user with that email exists it returns an error.
/// SO this function just makes sure that a user with that email does not exist.
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


/// This function gets a particular user by his id.
pub async fn get_user_by_id(executor: &Executor, id: &Id) -> Result<User> {
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


/// This function deletes a user by id.
pub async fn delete_user_by_id(executor: &Executor, id: &Id) -> Result<()> {
    query!("DELETE FROM users WHERE id = $1;", &id.bytes()).execute(executor).await?;
    Ok(())
}


/// update user by id
pub async fn update_user_by_id<'a>(executor: &Executor, id: &Id, map: &HashMap<&'a str, Value>) -> Result<User> {
    let mut index = 1usize;
    let mut updates = Vec::new();
    let mut values = Vec::new();
    for (key, value) in map {
        updates.push(format!("{} = ${}", key, index));
        values.push(value);
        index+=1;
    }
    if updates.len() == 0 {
        return  Err(Error::Custom(StatusCode::BAD_REQUEST, "No Data to Update. Please provide fields and values to be updated".into()));
    }
    let statement = format!("WITH updated AS (UPDATE users SET {} WHERE id = ${} RETURNING id) SELECT * FROM users_view WHERE id IN (SELECT id FROM updated);", updates.join(", "), index);
    let mut query = query_as::<Postgres, User>(&statement);
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(id);
    let user = query.fetch_one(executor).await?;
    Ok(user)
}