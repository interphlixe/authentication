use sqlx::{Postgres, Pool, query_as, FromRow};
use super::{User, Id};

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn create_user(executor: &Executor, user: User) -> Result<User> {
    let id = user.id.bytes();
    let email = serde_json::json!(user.email);
    let created_user = query_as(r#"
    INSERT INTO users 
    (id, email, user_name, first_name, last_name, password, created_at, profile_picture)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING id, email, user_name, first_name, last_name, password, created_at, profile_picture;
    "#,)
    .bind(id).bind(email).bind(user.user_name).bind(user.first_name).bind(user.last_name).bind(user.password).bind(user.created_at).bind(user.profile_picture)
    .fetch_one(executor).await?;
    Ok(created_user)
}