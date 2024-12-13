use actix_web::{http::StatusCode, web::{Data, Json, Path}, HttpResponse};
use crate::User;
use crate::user;
use super::*;

#[post("/signup")]
async fn signup(user: Json<User>, data: Data<Pool<Postgres>>) -> Result<impl Responder> {
    let executor = data.get_ref();
    let user = user.into_inner();
    let created_user = user::create_user(executor, user).await?;
    Ok(HttpResponse::Ok().json(created_user))
}


#[get("/users/{id}")]
async fn get_user(id: Path<String>,data: Data<Pool<Postgres>>) -> Result<impl Responder> {
    let id = id.into_inner();
    let id = id.as_str().parse().map_err(|_|{Error::Custom(StatusCode::BAD_REQUEST, "invalid id".into())})?;
    let executor = data.get_ref();
    let user = user::get_user_by_id(executor, id).await?;
    Ok(HttpResponse::Ok().json(user))
}