use actix_web::{http::StatusCode, web::{Data, Json, Path}, HttpResponse, delete, put};
use crate::{User, Value, Mailer};
use std::collections::HashMap;
use serde_json::json;
use argon2::Argon2;
use crate::user;
use super::*;

#[post("/signup")]
async fn signup(user: Json<User>, data: Data<(Db, Mailer, Argon2<'_>)>, req: HttpRequest) -> Result<impl Responder> {
    let executor = &data.0;
    let user = user.into_inner();
    let mailer = &data.1;
    let mail_config = &crate::config::Config::read().await.map_err(|e| Error::Custom(StatusCode::INTERNAL_SERVER_ERROR, e.into()))?.mail;
    let scheme = req.headers().get("X-Forwarded-Proto").and_then(|v| v.to_str().ok()).unwrap_or("http");
    let host = req.headers().get("Host").and_then(|v| v.to_str().ok()).unwrap_or("localhost");
    let created_user = user::signup(executor, user, mailer, mail_config, scheme, host).await?;
    Ok(HttpResponse::Created().json(created_user))
}


#[get("/users/{id}")]
async fn get_user(id: Path<String>, data: Data<(Db, Mailer, Argon2<'_>)>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    let user = user::get_user_by_id(executor, &id).await?;
    Ok(HttpResponse::Ok().json(user))
}


#[delete("/users/{id}")]
async fn delete_user(id: Path<String>, data: Data<(Db, Mailer, Argon2<'_>)>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    user::delete_user_by_id(executor, &id).await?;
    Ok(HttpResponse::Ok().json(json!("user delted successfully")))
}


#[put("/users/{id}")]
async fn update_user(id: Path<String>, data: Data<(Db, Mailer, Argon2<'_>)>, map: Json<HashMap<String, Value>>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = &data.0;
    let map = map.0;
    let user = user::update_user_by_id(executor, &id, map).await?;
    Ok(HttpResponse::Ok().json(json!(user)))
}
