use actix_web::{http::StatusCode, web::{Data, Json, Path}, HttpResponse, delete, put};
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::User;
use crate::user;
use super::*;

#[post("/signup")]
async fn signup(user: Json<User>, data: Data<(Db, Mailer)>) -> Result<impl Responder> {
    let executor = &data.0;
    let user = user.into_inner();
    let created_user = user::create_user(executor, user).await?;
    Ok(HttpResponse::Created().json(created_user))
}


#[get("/users/{id}")]
async fn get_user(id: Path<String>, data: Data<(Db, Mailer)>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    let user = user::get_user_by_id(executor, &id).await?;
    Ok(HttpResponse::Ok().json(user))
}


#[delete("/users/{id}")]
async fn delete_user(id: Path<String>, data: Data<(Db, Mailer)>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    user::delete_user_by_id(executor, &id).await?;
    Ok(HttpResponse::Ok().json(json!("user delted successfully")))
}


#[put("/users/{id}")]
async fn update_user(id: Path<String>, data: Data<(Db, Mailer)>, map: Json<HashMap<String, Value>>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    let map = map.0;
    let user = user::update_user_by_id(executor, &id, map).await?;
    Ok(HttpResponse::Ok().json(json!(user)))
}