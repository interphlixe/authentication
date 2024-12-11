#![allow(unused)]
mod server;
mod domain;
mod db;

/// re-export all the puliclic contents of the `db` module.
use db::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    Ok(server::start().await?)
}
