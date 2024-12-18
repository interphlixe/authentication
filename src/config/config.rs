use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use tokio::fs::read_to_string;
use std::env::var;
use super::*;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub mail: Mail,
    pub database: Database
}


impl Config {
    pub async fn read() -> Result<Self> {
        let path = var("CONFIG").unwrap_or(String::from("./config.json"));
        let json = read_to_string(path).await?;
        Ok(serde_json::from_str(&json)?)
    }
}