use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use lettre::{message::Mailbox, transport::smtp::PoolConfig, AsyncSmtpTransport, Tokio1Executor};
use super::*;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;
type Mailer = AsyncSmtpTransport<Tokio1Executor>;


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
}