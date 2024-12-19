use lettre::{Message, message::Mailbox, AsyncTransport};
use std::env::var;
use crate::Mailer;


pub async fn send_mail(mailer: Mailer, sender: Mailbox, receiver: Mailbox, subject: &str, message: String) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
    .from(sender)
    .to(receiver)
    .subject(subject)
    .body(message)?;

    mailer.send(email).await?;
    Ok(())
}