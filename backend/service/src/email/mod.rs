use std::time::Duration;

use chrono::Utc;
use lettre::address::AddressError;
use lettre::message::{Mailbox, MultiPart};
use lettre::Message;
use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    SmtpTransport, Transport,
};

use repository::{EmailQueueRowRepository, EmailQueueStatus, RepositoryError};

use crate::service_provider::ServiceContext;
use crate::settings::Settings;

pub mod enqueue;

pub static MAX_RETRIES: i32 = 3;

// We use a trait for EmailService to allow mocking in tests
pub trait EmailServiceTrait: Send + Sync {
    fn test_connection(&self) -> Result<bool, EmailServiceError>;

    fn send_queued_emails(&self, ctx: &ServiceContext) -> Result<usize, EmailServiceError>;
}

pub struct EmailService {
    pub mailer: SmtpTransport,
    pub from: Mailbox,
    pub url: String,
}

#[derive(Debug)]
pub enum EmailServiceError {
    GenericError(String),
    AddressError(AddressError),
    LettreError(lettre::error::Error),
    SmtpError(lettre::transport::smtp::Error),
    DatabaseError(RepositoryError),
}

impl From<RepositoryError> for EmailServiceError {
    fn from(error: RepositoryError) -> Self {
        EmailServiceError::DatabaseError(error)
    }
}

impl EmailService {
    pub fn new(settings: Settings) -> Self {
        let mut transport_builder =
            SmtpTransport::builder_dangerous(settings.mail.host.clone()).port(settings.mail.port);

        if settings.mail.starttls {
            let tls_parameters = TlsParameters::new(settings.mail.host);
            match tls_parameters {
                Ok(tls_parameters) => {
                    transport_builder = transport_builder.tls(Tls::Required(tls_parameters));
                }
                Err(error) => {
                    panic!("EmailService error creating tls parameters {}", error);
                }
            }
        }

        if !settings.mail.username.is_empty() && !settings.mail.password.is_empty() {
            let credentials = Credentials::new(
                settings.mail.username.clone(),
                settings.mail.password.clone(),
            );
            transport_builder = transport_builder.credentials(credentials);
        }

        let mailer = transport_builder.build();

        EmailService {
            mailer,
            from: settings
                .mail
                .from
                .parse()
                .expect("The configured mail:from address is not valid"), // This could panic on startup, but only if an invalid from address is configured
            url: settings.server.app_url,
        }
    }
}

impl EmailServiceTrait for EmailService {
    fn test_connection(&self) -> Result<bool, EmailServiceError> {
        self.mailer
            .test_connection()
            .map_err(|e| EmailServiceError::SmtpError(e))
    }

    fn send_queued_emails(&self, ctx: &ServiceContext) -> Result<usize, EmailServiceError> {
        log::debug!("Sending queued emails");

        let repo = EmailQueueRowRepository::new(&ctx.connection);
        let queued_emails = repo.un_sent()?;
        let mut error_count = 0;
        let mut sent_count = 0;

        for mut email in queued_emails {
            let to = match email.to_address.parse() {
                Ok(to) => to,
                Err(e) => {
                    email.error = Some(format!(
                        "Unable to parse email address {} - {}",
                        email.to_address, e
                    ));
                    email.status = EmailQueueStatus::Failed;
                    email.updated_at = Utc::now().naive_utc();
                    repo.update_one(&email)?;
                    error_count += 1;
                    continue;
                }
            };

            let message = Message::builder()
                .to(to)
                .from(self.from.clone())
                .subject(email.subject.clone())
                .multipart(MultiPart::alternative_plain_html(
                    email.text_body.clone(),
                    email.html_body.clone(),
                ));

            let message = match message {
                Ok(message) => message,
                Err(e) => {
                    email.error = Some(format!("Unable to create email - {:?}", e));
                    email.status = EmailQueueStatus::Failed;
                    email.updated_at = Utc::now().naive_utc();
                    repo.update_one(&email)?;
                    error_count += 1;
                    continue;
                }
            };

            let result = self.mailer.send(&message);

            match result {
                Ok(_) => {
                    // Successfully Sent
                    email.error = None;
                    email.status = EmailQueueStatus::Sent;
                    email.sent_at = Some(Utc::now().naive_utc());
                    email.updated_at = Utc::now().naive_utc();
                    repo.update_one(&email)?;
                    sent_count += 1;
                }
                Err(send_error) => {
                    // Failed to send
                    email.updated_at = Utc::now().naive_utc();

                    if email.retries >= MAX_RETRIES {
                        log::error!(
                            "Failed to send email {} to {} after {} retries - {}",
                            email.id,
                            email.to_address,
                            MAX_RETRIES,
                            send_error
                        );
                        email.error = Some(format!(
                            "Failed to send email after {} retries - {}",
                            MAX_RETRIES, send_error
                        ));
                        email.status = EmailQueueStatus::Failed;
                    } else if send_error.is_permanent() {
                        log::error!(
                            "Permanently failed to send email {} to {}",
                            email.id,
                            email.to_address,
                        );
                        email.error =
                            Some(format!("Failed to send email permanently - {}", send_error));
                        email.status = EmailQueueStatus::Failed;
                    } else {
                        log::error!(
                            "Temporarily failed to send email {} to {} - {}",
                            email.id,
                            email.to_address,
                            send_error
                        );
                        email.error = Some(format!("Failed to send email - {}", send_error));
                        email.status = EmailQueueStatus::Errored;
                        email.retries += 1;
                    }

                    error_count += 1;
                    repo.update_one(&email)?;

                    continue;
                }
            }
        }

        if error_count > 0 {
            return Err(EmailServiceError::GenericError(format!(
                "Failed to send {} emails",
                error_count
            )));
        }

        log::debug!("Sent {} emails", sent_count);

        Ok(sent_count)
    }
}

static TASK_INTERVAL: Duration = Duration::from_secs(10);

pub async fn periodically_send_queued_emails(
    email_service: &Box<dyn EmailServiceTrait>,
    service_context: ServiceContext,
) {
    let mut interval = tokio::time::interval(TASK_INTERVAL);
    loop {
        interval.tick().await;
        log::debug!("Sending emails");
        let send_emails = email_service.send_queued_emails(&service_context);
        match send_emails {
            Ok(num) => {
                if num > 0 {
                    log::info!("Sent {} queued emails", num);
                }
            }
            Err(error) => log::error!("Error sending queued emails: {:?}", error),
        };
    }
}

#[cfg(test)]
#[cfg(feature = "email-tests")]
mod email_test {

    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;
    use repository::mock::MockDataInserts;
    use repository::test_db::setup_all;

    #[actix_rt::test]
    async fn test_email_connection() {
        let (_, _, connection_manager, _) =
            setup_all("test_email_connection", MockDataInserts::none()).await;

        let service_provider = ServiceProvider::new(connection_manager, get_test_settings(""));
        let email_service = service_provider.email_service;
        let test = email_service.test_connection().unwrap();
        assert!(test);
    }
}
