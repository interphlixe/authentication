use actix_web::http::StatusCode;
use sqlx::{query, query_as, Error as SqlxError, Execute, FromRow, Pool, Postgres};
use super::{db, EmailAddress, Error, Id, User};
use std::collections::HashMap;
use serde_json::{Number, Value};

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
        if let Some(value) = option_value(map.remove(field)) {
            new_map.insert(field, value);
        }
    }
    Ok(db::user::update_user_by_id(executor, id, &new_map).await?)
}


/// This function returns true if value is not Empty else it returns false.
fn value_is_not_empty(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(_) => true,
        Value::Number(number) => Number::from_u128(0).unwrap() != *number || Number::from_i128(0).unwrap() != *number || Number::from_f64(0.0).unwrap() != *number,
        Value::String(value) => !value.is_empty(),
        Value::Array(value) => value.len() != 0,
        Value::Object(map) => map.len() != 0,
    }
}

/// This function takes in a value of `Option<Value>` and returns `Option<Value>`
/// this function returns `None` if the option is `None`
/// If the option has value. It uses the `value_is_not_empty` function to check if the value is Empty.
/// if the Value is empty it also returns `None` else it returns the Value.
fn option_value(option: Option<Value>) -> Option<Value> {
    match option {
        None => None,
        Some(value) => if value_is_not_empty(&value){Some(value)}else{None}
    }
}