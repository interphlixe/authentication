use actix_web::http::StatusCode;
use sqlx::{query, query_as, Error as SqlxError, Execute, FromRow, Pool, Postgres};
use super::{db, EmailAddress, Error, Id, User, Value};
use std::collections::HashMap;

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

pub async fn create_user(executor: &Executor, mut user: User) -> Result<User> {
    db::user::create_user(executor, &user).await?;
    user.password = Default::default();
    Ok(user)
}


pub async fn get_user_by_id(executor: &Executor, id: &Id) -> Result<User> {
    Ok(db::user::get_user_by_id(executor, id).await?)
}


pub async fn delete_user_by_id(executor: &Executor, id: &Id) -> Result<()> {
    Ok(db::user::delete_user_by_id(executor, id).await?)
}

/// update the `user_name`, `first_name` and `last_name` of a user with the given Id.
pub async fn update_user_by_id(executor: &Executor, id: &Id, mut map: HashMap<String, Value>) -> Result<User> {
    let fields = ["user_name", "first_name", "last_name"];
    let mut new_map = HashMap::new();
    for field in fields {
        if let Some(value) = Value::as_option_from_option(map.remove(field)) {
            new_map.insert(field, value);
        }
    }
    Ok(db::user::update_user_by_id(executor, id, &new_map).await?)
}