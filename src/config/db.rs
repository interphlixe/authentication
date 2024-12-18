use sqlx::{Pool, Postgres, Error, query};
use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use url::Url;
use super::*;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;


const DEFAULT_DB_NAME: &'static str = "postgres";
const DB_NAME: &'static str = "interphlix";


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Database {
    pub credentials: Option<Credentials>,
    pub name: String,
    pub url: String,
}


impl Default for Database {
    fn default() -> Self {
        let credentials = None;
        let name = String::from(DB_NAME);
        let url = String::from("postgres://localhost:5432");
        Self {credentials, name, url}
    }
}