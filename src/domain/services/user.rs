use sqlx::{query, query_as, Error as SqlxError, Execute, FromRow, Pool, Postgres};
use super::{db, EmailAddress, Error, Id, User, Value, Mailer, Verification};
use crate::domain::services::verification::generate_verification_code;
use crate::domain::services::mail::send_html_email;
use actix_web::http::StatusCode;
use std::collections::HashMap;
use lettre::message::Mailbox;
use crate::config::Mail;

type Result<T> = std::result::Result<T, Error>;
type Executor = Pool<Postgres>;


pub async fn signup(executor: &Executor, mut user: User, mailer: &Mailer, mail_config: &Mail, scheme: &str, host: &str) -> Result<User> {
    db::user::create_user(executor, &user).await?;
    user.password = Default::default();

    // Generate a verification code for the new user
    let verification = generate_verification_code(executor, user.id.clone()).await?;

    // Include the HTML template
    const HTML_TEMPLATE: &'static str = include_str!("mail.html");

    // Prepare the magic link and replace placeholders
    let magic_link = format!("{}://{}/magic-link/{}", scheme, host, verification.id.simple().to_string());
    let message = HTML_TEMPLATE
        .replace("{{magic_link}}", &magic_link)
        .replace("{{code}}", &verification.code);

    // Send the verification email
    let subject = "Verification";
    let name = Some(user.user_name.clone());
    let email = user.email.clone().into();
    let receiver = Mailbox{name, email};
    send_html_email(
        mailer,
        mail_config.sender.clone(),
        receiver,
        subject,
        message,
    ).await.map_err(|_|"error could not send verification email to the provided email address");

    Ok(user)
}


pub async fn get_user_by_id(executor: &Executor, id: &Id) -> Result<User> {
    Ok(db::user::get_user_by_id(executor, id).await?)
}


pub async fn delete_user_by_id(executor: &Executor, id: &Id) -> Result<()> {
    Ok(db::user::delete_user_by_id(executor, id).await?)
}

/// update the `user_name`, `first_name` and `last_name` of a user with the given Id.
pub async fn update_user_by_id(executor: &Executor, id: &Id, mut map: HashMap<String, Value>) -> Result<User> {
    let fields = ["user_name", "first_name", "last_name"];
    let mut new_map = HashMap::new();
    for field in fields {
        if let Some(value) = Value::as_option_from_option(map.remove(field)) {
            new_map.insert(field, value);
        }
    }
    Ok(db::user::update_user_by_id(executor, id, &new_map).await?)
}
