use sqlx::postgres::Postgres;
use tokio::sync::OnceCell;
use static_init::dynamic;
use sqlx::pool::Pool;
use std::env::var;
use sqlx::Error;
use sqlx::query;
use super::*;

const CREATE_DATABASE_SATATEMENT: &'static str = "CREATE DATABASE";


const CREATE_USERS_TABLE_STATEMENT: &'static str = r#"
CREATE TABLE users (
     id BYTEA PRIMARY KEY,
     email JSONB NOT NULL,
     user_name TEXT NOT NULL,
     first_name TEXT NOT NULL,
     last_name TEXT NOT NULL,
     password TEXT NOT NULL,
     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
 );
"#;


const ALTER_USERS_TABLE_STATEMENT: &'static str = r#"
ALTER TABLE users
ADD COLUMN IF NOT EXISTS id BYTEA PRIMARY KEY,
ADD COLUMN IF NOT EXISTS email JSONB NOT NULL,
ADD COLUMN IF NOT EXISTS user_name TEXT NOT NULL,
ADD COLUMN IF NOT EXISTS first_name TEXT NOT NULL,
ADD COLUMN IF NOT EXISTS last_name TEXT NOT NULL,
ADD COLUMN IF NOT EXISTS password TEXT NOT NULL,
ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
"#;


const EMAIL_INDEX_ON_USERS_TABLE_STATEMENT: &'static str = r#"
DO $$
 BEGIN
     IF NOT EXISTS (
         SELECT 1
         FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE c.relname = 'users_email_index'
         AND n.nspname = 'public'
     ) THEN
         EXECUTE 'CREATE UNIQUE INDEX users_email_index ON users
 ((email->>''email''))';
     END IF;
 END $$;
"#;


/// These are names of the env variable names they are not actual values.
/// the values of these variables will be used as keys to extract values from ENV variables.
const DATABASE_URL: &'static str = "DATABASE_URL";
const DEFAULT_DB_NAME: &'static str = "postgres";
const USER_NAME: &'static str = "USER_NAME";
const PASSWORD: &'static str = "PASSWORD";
const DB_NAME: &'static str = "DB_NAME";


const ERROR_CODE_DB_DOES_NOT_EXIST: &'static str = "3D000";
const ERROR_CODE_TABLE_EXISTS: &'static str = "42P07";

/// This functions creates a connection to the database and returns the pool.
/// If the database does not exist. then it will connect to the default database. Then create the needed
/// database and connect to it and return
/// This function implements recussion using a while loop and a queue to avoid the use of `Box::pin`
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
                        // check if the returned error indicates that the database does not exist.
                        if err.code().as_deref() == Some(ERROR_CODE_DB_DOES_NOT_EXIST) {
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
        Some(result) => {
            match result {
                Ok(pool) => {
                    // making sure that the users table is always created.
                    create_users_table(&pool).await?;
                    Ok(pool)
                },
                Err(err) => Err(err.into())
            }
        },
        None => Err("Unknown error".into())
    }
}

/// this function create the given database
async fn create_db(name: &str, url: &str) -> Result<()> {
    let pool = Pool::<Postgres>::connect(url).await?;
    let sql = format!("{} {}", CREATE_DATABASE_SATATEMENT, name);
    query(&sql).execute(&pool).await?;
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


/// Create the Table users and if the table already exists. make sure all columns as upto date.
pub async fn create_users_table(pool: &Pool<Postgres>) -> Result<()> {
    let sql = CREATE_USERS_TABLE_STATEMENT;
    match query(sql).execute(pool).await {
        Ok(result) => create_users_index(pool).await,
        Err(err) => {
            match err.as_database_error() {
                Some(db_err) => {
                    match db_err.code().as_deref() {
                        // check if the error is indicating that the table already exists.
                        Some(ERROR_CODE_TABLE_EXISTS) => alter_users_table(pool).await,
                        _ => Err(err.into())
                    }
                }
                _ => Err(err.into()),
            }
        }
    }
}


/// This function adds columns to the table if they are missing.
/// It also invokes the function to index the email column on the users table.
pub async fn alter_users_table(pool: &Pool<Postgres>) -> Result<()> {
    let sql = ALTER_USERS_TABLE_STATEMENT;
    query(sql).execute(pool).await?;
    create_users_index(pool).await?;
    Ok(())
}

/// This function creates an index of the email column for the users table.
pub async fn create_users_index(pool: &Pool<Postgres>) -> Result<()> {
    let sql = EMAIL_INDEX_ON_USERS_TABLE_STATEMENT;
    query(sql).execute(pool).await?;
    Ok(())
}