use sqlx::postgres::Postgres;
use tokio::sync::OnceCell;
use static_init::dynamic;
use sqlx::pool::Pool;
use std::env::var;
use super::*;

#[dynamic]
static CONNECTION_STRING: String = var("DATABASE_URL").unwrap_or(String::from("postgres://localhost:5432"));
#[dynamic]
static DB_NAME: String = var("DB_NAME").unwrap_or(String::from("Authentication"));

static CELL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

/// get a reference to the global database pool.
pub async fn pool() -> Result<&'static Pool<Postgres>> {
    CELL.get_or_try_init(init).await
}

/// return a new insatnce of the pool.
async fn init() -> Result<Pool<Postgres>> {
    Ok(Pool::connect(&CONNECTION_STRING).await?)
}