use actix_web::{HttpServer, App, Responder, web, get, post};
use sqlx::{Pool, Postgres};
use static_init::dynamic;
use super::{init, Error};
use user::*;

mod user;


type Result<T> = std::result::Result<T, Error>;

#[dynamic]
static PORT: u16 = read_port("PORT").unwrap_or(8080);

///Start a new Http server.
pub async fn start() -> super::Result<()> {
    let db = init().await?;
    let data = web::Data::new(db);
    HttpServer::new(move|| {
        App::new().app_data(data.clone()).service(hello).service(signup)
    })
    .bind(("127.0.0.1", *PORT))?
    .run()
    .await?;
    Ok(())
}

/// This function reads the posrt to be used from the environment variable with the given key.
/// if No value was set it returns None.
/// if the value set could not be converted to an int. also returns None.
fn read_port(key: &'static str) -> Option<u16> {
    match std::env::var(key) {
        Err(_) => None,
        Ok(value) => {
            match value.parse::<u16>() {
                Err(_) => None,
                Ok(value) => Some(value)
            }
        }
    }
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("<h1>Hello {name}</h1>")
}