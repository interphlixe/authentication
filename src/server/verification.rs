use actix_web::{get, patch, web::{Data, Path, Query}, HttpResponse, Responder, http::StatusCode};
use crate::{verification, Error, Mailer};
use serde::Deserialize;
use sqlx::types::Uuid;
use crate::Id;
use super::*;



#[derive(Deserialize)]
struct VerifyQuery {
    code: String,
}


#[get("/magic-link/{id}")]
async fn verify_magic_link(id: Path<String>, data: Data<(Db, Mailer)>) -> Result<impl Responder> {
    let id_str = id.into_inner();
    let verification_id = id_str.parse::<Uuid>().map_err(|_| {
        Error::Custom(StatusCode::BAD_REQUEST, "Invalid UUID format".into())
    })?;

    let executor = &data.0;
    let updated_user = verification::verify_magic_link(executor, &verification_id).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}

#[patch("/users/verify-email/{id}")]
async fn verify_user(
    id: Path<String>,
    query: Query<VerifyQuery>,
    data: Data<(Db, Mailer)>,
) -> Result<impl Responder> {
    let id_str = id.into_inner();
    let user_id = id_str.parse::<Id>().map_err(|_| {
        Error::Custom(StatusCode::BAD_REQUEST, "Invalid ID format".into())
    })?;

    let code = &query.code;

    let executor = &data.0;
    let updated_user = verification::verify_code_and_update_user(executor, user_id, code).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}
