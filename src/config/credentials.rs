use lettre::transport::smtp::authentication::Credentials as Cred;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub password: String
}


impl From<&Credentials> for Cred {
    fn from(credentials: &Credentials) -> Self {
        Cred::from((&credentials.name, &credentials.password))
    }
}