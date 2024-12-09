use crate::User;
use crate::user;
use super::*;

#[post("/signup")]
async fn signup(user: web::Json<User>, data: web::Data<Pool<Postgres>>) -> Result<impl Responder> {
    let executor = data.get_ref();
    let user = user.into_inner();
    let created_user = user::create_user(executor, user).await?;
    Ok(serde_json::to_string(&created_user))
}