use actix_web::{get, web::{Data, Path}, HttpResponse, Responder, http::StatusCode};
use crate::{verification, Error, Mailer};
use sqlx::types::Uuid;
use super::*;

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
