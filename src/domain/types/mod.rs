mod email_address;
mod number;
mod value;
mod error;
mod user;
mod id;

pub use email_address::*;
pub use number::*;
pub use value::*;
pub use error::*;
pub use user::*;
pub use id::*;


pub type Mailer = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;