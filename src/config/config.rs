use tokio::fs::{read_to_string, write};
use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use std::io::ErrorKind;
use std::env::var;
use super::*;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub mail: Mail,
    pub database: Database,
    pub argon: Argon2Config
}


impl Config {
    pub async fn read() -> Result<Self> {
        let path = var("CONFIG").unwrap_or(String::from("./config.json"));
        let json = match read_to_string(&path).await {
            Ok(json) => json,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        let config = Config{mail: Mail::from_env()?, database: Default::default(), argon: Default::default()};
                        config.write(&path).await?;
                        return Ok(config)
                    },
                    _ => Err(err)?
                }
            }
        };
        Ok(serde_json::from_str(&json)?)
    }

    async fn write(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        let contents = json.as_bytes();
        Ok(write(path, contents).await?)
    }
}