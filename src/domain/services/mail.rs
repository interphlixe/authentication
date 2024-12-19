use lettre::{Message, message::{Mailbox, SinglePart}, AsyncTransport};
use std::env::var;
use crate::Mailer;


pub async fn send_mail(mailer: Mailer, sender: Mailbox, receiver: Mailbox, subject: &str, message: String) -> Result<(), Box<dyn std::error::Error>> {
    let body = SinglePart::html(message);
    let email = Message::builder()
    .from(sender)
    .to(receiver)
    .subject(subject)
    .singlepart(body)?;

    mailer.send(email).await?;
    Ok(())
}