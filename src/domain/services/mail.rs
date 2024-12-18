use lettre::{
    message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::PoolConfig
};
use std::env::var;


type Mailer = AsyncSmtpTransport<Tokio1Executor>;


pub async fn send_mail(mailer: Mailer, sender: Mailbox, receiver: Mailbox, subject: &str, message: String) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
    .from(sender)
    .to(receiver)
    .subject(subject)
    .body(message)?;

    mailer.send(email).await?;
    Ok(())
}