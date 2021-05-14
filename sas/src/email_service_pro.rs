use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error;
use lettre::smtp::response;
use lettre::{ClientSecurity, SmtpClient, SmtpTransport, Transport};
use lettre_email::{EmailBuilder, Mailbox};

#[allow(dead_code)]
pub struct InsecureEmailHelper {
    username: String,
    password: String,
    host: String,
    port: u16,
    pub sender: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub subject: Option<String>,
    pub body: Option<String>,
    mailer: Option<SmtpTransport>,
}

impl InsecureEmailHelper {
    #[allow(dead_code)]
    pub fn new(username: String, password: String, host: String, port: u16) -> Self {
        InsecureEmailHelper {
            username,
            password,
            host,
            port,
            sender: None,
            recipients: None,
            subject: None,
            body: None,
            mailer: None,
        }
    }

    #[allow(dead_code)]
    pub fn add_sender(&mut self, sender: String) {
        self.sender = Some(sender);
    }

    #[allow(dead_code)]
    pub fn add_recipients(&mut self, recipients: Vec<String>) {
        self.recipients = Some(recipients);
    }

    #[allow(dead_code)]
    pub fn add_subject(&mut self, subject: String) {
        self.subject = Some(subject);
    }

    #[allow(dead_code)]
    pub fn add_body(&mut self, body: String) {
        let b = match &self.body {
            Some(t) => format!("{}\n{}", t, body),
            None => body,
        };

        self.body = Some(b);
    }

    fn build(&mut self) {
        let creds = Credentials::new(self.username.clone(), self.password.clone());

        let mailer = SmtpClient::new((&self.host[..], self.port), ClientSecurity::None)
            .unwrap()
            .credentials(creds)
            .authentication_mechanism(Mechanism::Plain)
            .transport();

        self.mailer = Some(mailer);
    }

    #[allow(dead_code)]
    pub fn send(&mut self) -> Result<response::Response, error::Error> {
        if let Self {
            sender: Some(sdr),
            recipients: Some(rec),
            subject: sub @ Some(_),
            body: bdy @ Some(_),
            ..
        } = self
        {
            let mut email = EmailBuilder::new().from(Mailbox::new(sdr.clone()));

            for i in rec.iter() {
                email = email.to(i.clone());
            }

            let email = email
                .subject(sub.take().unwrap())
                .body(bdy.take().unwrap())
                .build()
                .unwrap();

            self.build();

            self.mailer.take().unwrap().send(email.into())
        } else {
            panic!("Missing one of the fields!");
        }
    }
}

#[test]
fn insecure_email_helper_test() {
    let username = "username@example.com".to_string();
    let password = "password".to_string();
    let host = "mail.example.com".to_string();
    let port = 25;
    let mut foo = InsecureEmailHelper::new(username.clone(), password, host, port);

    foo.add_subject("Invitation from Rust!".to_string());
    foo.add_body("test 1".to_string());
    foo.add_body("test 2".to_string());

    foo.add_sender(username);
    foo.add_recipients(vec!["someone@example.com".to_string()]);

    foo.send().expect("233");

    foo.add_subject("Invitation from Rust! Again!".to_string());
    foo.add_body("test 3".to_string());
    foo.add_body("test 4".to_string());

    foo.send().expect("111");
}
