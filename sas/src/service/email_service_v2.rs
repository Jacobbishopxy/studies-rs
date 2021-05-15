use crate::error::ServiceError;
use crate::model::Invitation;
use crate::util::{InsecureEmailHelper, CFG};

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    let username = CFG.get("SMTP_USERNAME").unwrap().to_string();
    let password = CFG.get("SMTP_PASSWORD").unwrap().to_string();
    let host = CFG.get("SMTP_HOST").unwrap().to_string();
    let port = CFG.get("SMTP_PORT").unwrap().parse::<u16>().unwrap();
    let mut email_helper = InsecureEmailHelper::new(username.clone(), password, host, port);

    let subject = "You have been invited to join Simple-Auth-Server Rust".to_string();
    let body = format!(
        "
        Please click on the link below to complete registration. <br/>
        <a href=\"http://localhost:3000/register.html?id={}&email={}\">
        http://localhost:3030/register</a> <br/>
        your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    let recipients = vec![invitation.email.clone()];
    let result = email_helper
        .add_subject(subject)
        .add_body(body)
        .add_sender(username)
        .add_recipients(recipients)
        .send();

    match result {
        Ok(res) => {
            println!("Send Email Succeed: \n {:#?}", res);
            Ok(())
        }
        Err(error) => {
            println!("Send Email Error: \n {:#?}", error);
            Err(ServiceError::InternalServerError)
        }
    }
}
