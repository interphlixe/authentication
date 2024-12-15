use lettre::{
    message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::PoolConfig
};
use std::env::var;


pub async fn send_mail(receiver: Mailbox, subject: &str, message: String) -> Result<(), Box<dyn std::error::Error>> {
    let mailer = mailer();
    let sender = mail_sender();
    let email = Message::builder()
    .from(sender)
    .to(receiver)
    .subject(subject)
    .body(message)?;

    mailer.send(email).await?;
    Ok(())
}


fn mailer() -> AsyncSmtpTransport<Tokio1Executor> {
    let url = smtp_url();
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::from_url(
        &url,
    )
    .expect("could not create smtp connection")
    .pool_config(PoolConfig::new())
    .build();
    mailer
}


fn mail_sender() -> Mailbox {
    let name = var("EMAIL_SENDER_NAME").ok();
    let email = match var("EMAIL_SENDER_ADDRESS") {
        Ok(value) => value.as_str().parse().expect("invalid EMAIL_SENDER_ADDRESS"),
        Err(_) => panic!("make sure to set the EMAIL_SENDER_ADDRESS")
    };
    Mailbox{name, email}
}


fn smtp_url() -> String {
    var("SMTP_URL").expect("make sure to set SMTP_URL")
}