use sqlx::postgres::Postgres;
use tokio::sync::OnceCell;
use static_init::dynamic;
use sqlx::pool::Pool;
use std::env::var;
use super::*;
use sqlx::Error;



/// These are names of the env variable names they are not actual values.
/// the values of these variables will be used as keys to extract values from ENV variables.
const DATABASE_URL: &'static str = "DATABASE_URL";
const DEFAULT_DB_NAME: &'static str = "postgres";
const USER_NAME: &'static str = "USER_NAME";
const PASSWORD: &'static str = "PASSWORD";
const DB_NAME: &'static str = "DB_NAME";

/// This functions creates a connection to the database and returns the pool.
/// If the database does not exist. then it will connect to the default database. Then create the needed
/// database and connect to it and return
pub async fn init() -> Result<Pool<Postgres>> {
    let mut result: Option<Result<Pool<Postgres>>> = None;
    let mut uris: std::collections::VecDeque<url::Url> = Default::default();
    let uri = db_url();
    uris.push_back(uri);
    while let Some(uri) = uris.pop_front() {
        let url = uri.as_str();
        match Pool::connect(url).await {
            Ok(pool) => result = Some(Ok(pool)),
            Err(err) => {
                match err {
                    Error::Database(err) => {
                        if err.code().as_deref() == Some("3D000") {
                            let name = uri.path();
                            let name = name.replace("/", "");
                            let mut uri = uri.clone();
                            uri.set_path(&DEFAULT_DB_NAME);
                            let url = uri.as_str();
                            create_db(&name, url).await?;
                            uris.push_back(db_url());
                        }
                    },
                    _ => result = Some(Err(err.into()))
                }
            }
        }
    }
    match result {
        Some(result) => result,
        None => Err("Unknown error".into())
    }
}

/// this function create the given database
async fn create_db(name: &str, url: &str) -> Result<()> {
    let pool = Pool::<Postgres>::connect(url).await?;
    let sql = format!("CREATE DATABASE {}", name);
    sqlx::query(&sql).execute(&pool).await?;
    Ok(())
}


/// construct and validate the database_url from the environment variables.
fn db_url() -> url::Url {
    let mut url = url::Url::parse(&var(DATABASE_URL).unwrap_or(String::from("http://localhost:5432"))).expect(&format!("invalid {}. only the developer can resolve", DATABASE_URL));
    if  url.username() == "" {
        if let Ok(username) = var(USER_NAME) {
            url.set_username(&username);
        }
    }
    if url.password() == None {
        if let Ok(password) = var(USER_NAME) {
            url.set_password(Some(&password));
        }
    }
    let mut segments = url.path_segments().expect("invalid DATABASE_URL. url cannot be base");
    match segments.next() {
        Some(value) => {
            if value == "" {
                let db_name = var(DB_NAME).unwrap_or(DEFAULT_DB_NAME.into());
                url.set_path(&db_name);
            }
        }
        None => {
            let db_name = var(DB_NAME).unwrap_or(DEFAULT_DB_NAME.into());
            url.set_path(&db_name);
        }
    }
    url
}