#![allow(unused)]
mod server;
mod domain;
mod config;


use domain::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(server::start().await?)
}
