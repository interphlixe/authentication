use lettre::{message::Mailbox, transport::smtp::PoolConfig};
use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use std::env::var;
use crate::Mailer;
use super::*;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    ///These are the credentials for sending email
    /// The user_name and the password respectively
    pub credentials: Option<Credentials>,
    ///This is the url to the smtp server.
    /// Port should be included if necessary.
    pub url: String,
    ///The default sender's Mailbox.
    pub sender: Mailbox
}


impl Mail {
    pub fn mailer(&self) -> Result<Mailer> {
        let mut mailer = Mailer::from_url(&self.url)?;
        if let Some(credentials) = &self.credentials {
            mailer = mailer.credentials(credentials.into())
        }
        Ok(mailer.pool_config(PoolConfig::new()).build())
    }


    pub fn from_env() -> Result<Self> {
        let url = var("MAIL_URL").map_err(|_|"Set the MAIL_URL env variable.")?;
        let sender = var("MAIL_SENDER").map_err(|_|"Set the MAIL_SENDER env variable.")?;
        let sender = match serde_json::from_str(&sender) {
            Ok(sender) => sender,
            Err(_) => Mailbox::new(None, sender.parse().map_err(|_|"invalid MAIL_SENDER address")?)
        };
        let mut credentials = None;
        if let Ok(name) = var("MAIL_NAME") {
            if let Ok(password) = var("MAIL_PASSWORD") {
                credentials = Some(Credentials{name, password});
            }
        }
        Ok(Self{credentials, url, sender})
    }
}