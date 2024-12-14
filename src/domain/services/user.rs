use actix_web::http::StatusCode;
use sqlx::{query, query_as, Error as SqlxError, Execute, FromRow, Pool, Postgres};
use super::{User, Id, EmailAddress, Error};
use std::collections::HashMap;
use serde_json::Value;

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
    query!("DELETE FROM users WHERE id = $1;", &id.bytes()).execute(executor).await?;
    Ok(())
}


pub async fn update_user_by_id(executor: &Executor, id: Id, mut map: HashMap<String, Value>) -> Result<User> {
    let fields = ["user_name", "first_name", "last_name"];
    let mut updates = Vec::new();
    let mut values = Vec::new();
    let mut number = 1u8;
    for field in fields {
        match map.remove(field) {
            None => (),
            Some(value) => {
                match value.as_str() {
                    Some(value) => {
                        updates.push(format!("{} = ${}", field, number));
                        values.push(value.to_string());
                        number+=1;
                    },
                    None => return Err(Error::Custom(StatusCode::BAD_REQUEST, format!("expected a string from field: {}", field).into()))
                }
            }
        }
    }
    if values.len() == 0 {
        return Err(Error::Custom(StatusCode::BAD_REQUEST, "no data to update".into()))
    }
    let statement = format!("WITH updated AS (UPDATE users SET {} WHERE id = ${} RETURNING id) SELECT * FROM users_view WHERE id IN (SELECT id FROM updated);", updates.join(", "), number);
    let mut query = query_as::<Postgres, User>(&statement);
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(id);
    let user = query.fetch_one(executor).await?;
    Ok(user)
}