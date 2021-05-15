use crate::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error;
use lettre::smtp::response;
use lettre::{ClientSecurity, SmtpClient, SmtpTransport, Transport};
use lettre_email::{EmailBuilder, Mailbox};
use serde::{Deserialize, Serialize};

// 类型别名 Pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

/// 任何实现了 Into<String> 的类型可以被用于创建 Invitation
/// 这么做的目的是为了在使用 `&str`，`String`或 static str 时都可以生成 model
impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}

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
    pub fn add_sender(&mut self, sender: String) -> &mut Self {
        self.sender = Some(sender);
        self
    }

    #[allow(dead_code)]
    pub fn add_recipients(&mut self, recipients: Vec<String>) -> &mut Self {
        self.recipients = Some(recipients);
        self
    }

    #[allow(dead_code)]
    pub fn add_subject(&mut self, subject: String) -> &mut Self {
        self.subject = Some(subject);
        self
    }

    #[allow(dead_code)]
    pub fn add_body(&mut self, body: String) -> &mut Self {
        let b = match &self.body {
            Some(t) => format!("{}\n{}", t, body),
            None => body,
        };

        self.body = Some(b);
        self
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
