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



impl Database {
    const CREATE_DATABASE_STATEMENT: &'static str = "CREATE DATABASE";
    const CREATE_USERS_TABLE_STATEMENT: &'static str = r#"
        CREATE TABLE users (
            id BYTEA PRIMARY KEY,
            email JSONB NOT NULL,
            user_name TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            password TEXT NOT NULL,
            profile_picture TEXT,
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
        ADD COLUMN IF NOT EXISTS profile_picture TEXT,
        ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT
CURRENT_TIMESTAMP
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
    const ERROR_CODE_DB_DOES_NOT_EXIST: &'static str = "3D000";
    const ERROR_CODE_TABLE_EXISTS: &'static str = "42P07";

    pub async fn init(&self) -> Result<Pool<Postgres>> {
        let mut result: Option<Result<Pool<Postgres>>> = None;
        let mut uris: std::collections::VecDeque<Url> = Default::default();
        let uri = self.db_url();
        uris.push_back(uri);
        while let Some(uri) = uris.pop_front() {
            let url = uri.as_str();
            match Pool::connect(url).await {
                Ok(pool) => result = Some(Ok(pool)),
                Err(err) => {
                    match err {
                        Error::Database(err) => {
                            if err.code().as_deref() == Some(Self::ERROR_CODE_DB_DOES_NOT_EXIST) {
                                let name = uri.path().trim_start_matches('/');
                                let mut uri = uri.clone();
                                uri.set_path(DEFAULT_DB_NAME);
                                let url = uri.as_str();
                                self.create_db(name, url).await?;
                                uris.push_back(self.db_url());
                            }
                            result = Some(Err(err.into()))
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
                        self.create_users_table(&pool).await?;
                        Ok(pool)
                    },
                    Err(err) => Err(err.into())
                }
            },
            None => Err("Unknown error".into())
        }
    }

    async fn create_db(&self, name: &str, url: &str) -> Result<()> {
        let pool = Pool::<Postgres>::connect(url).await?;
        let sql = format!("{} {}", Self::CREATE_DATABASE_STATEMENT, name);
        query(&sql).execute(&pool).await?;
        Ok(())
    }

    fn db_url(&self) -> Url {
        let mut url = Url::parse(&self.url).expect("Invalid database URL");
        if url.username().is_empty() {
            if let Some(ref credentials) = self.credentials {
                url.set_username(&credentials.name).unwrap();
            }
        }
        if url.password().is_none() {
            if let Some(ref credentials) = self.credentials {
                url.set_password(Some(&credentials.password)).unwrap();
            }
        }
        if url.path().is_empty() || url.path() == "/" {
            url.set_path(&self.name);
        }
        url
    }

    pub async fn create_users_table(&self, pool: &Pool<Postgres>) -> Result<()> {
        let sql = Self::CREATE_USERS_TABLE_STATEMENT;
        match query(sql).execute(pool).await {
            Ok(_) => self.create_users_index(pool).await,
            Err(err) => {
                match err.as_database_error() {
                    Some(db_err) => {
                        if db_err.code().as_deref() == Some(Self::ERROR_CODE_TABLE_EXISTS) {
                            self.alter_users_table(pool).await
                        } else {
                            Err(err.into())
                        }
                    }
                    _ => Err(err.into()),
                }
            }
        }
    }

    pub async fn alter_users_table(&self, pool: &Pool<Postgres>) -> Result<()> {
        let sql = Self::ALTER_USERS_TABLE_STATEMENT;
        query(sql).execute(pool).await?;
        self.create_users_index(pool).await?;
        Ok(())
    }

    pub async fn create_users_index(&self, pool: &Pool<Postgres>) -> Result<()> {
        let sql = Self::EMAIL_INDEX_ON_USERS_TABLE_STATEMENT;
        query(sql).execute(pool).await?;
        Ok(())
    }
}