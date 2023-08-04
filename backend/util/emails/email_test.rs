#[cfg(test)]
pub mod email_test {
    #[cfg(feature = "email-tests")]
    pub fn send_test_emails(context: &ServiceContext) {
        context
            .service_provider
            .email_service
            .send_queued_emails(&context)
            .unwrap();
    }

    #[cfg(not(feature = "email-tests"))]
    pub fn send_test_emails(_context: &ServiceContext) {
        println!("Skipping email sending");
    }
}
