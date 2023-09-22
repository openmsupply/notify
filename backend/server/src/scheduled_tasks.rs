use service::{plugin::PluginTrait, service_provider::ServiceContext};
use std::time::Duration;

static TASK_INTERVAL: Duration = Duration::from_secs(10);

pub async fn scheduled_task_runner(
    service_context: ServiceContext,
    plugins: Vec<Box<dyn PluginTrait>>,
) {
    let mut interval = actix_web::rt::time::interval(TASK_INTERVAL);

    loop {
        interval.tick().await;
        log::debug!("Processing Scheduled Tasks");
        let send_emails = service_context
            .service_provider
            .email_service
            .send_queued_emails(&service_context);
        match send_emails {
            Ok(num) => {
                if num > 0 {
                    log::info!("Sent {} queued emails", num);
                }
            }
            Err(error) => log::error!("Error sending queued emails: {:?}", error),
        };

        // Process plugins
        // Note: If a plugin starts an infinite loop here, we're a bit stuffed as no more scheduled tasks will be processed.
        // Hopefully people will be smart enough not to do that?
        // Ideally this should be done in spawn_blocking thread so that it doesn't block the rt thread. https://github.com/openmsupply/notify/issues/133
        for plugin in &plugins {
            let result = plugin.tick(&service_context);
            if let Err(e) = result {
                log::error!("Error processing {} plugin: {:?}", plugin.name(), e);
            }
        }

        // Send Notifications
        let send_notifications = service_context
            .service_provider
            .notification_service
            .send_queued_notifications(&service_context)
            .await;
        match send_notifications {
            Ok(num) => {
                if num > 0 {
                    log::info!("Sent {} queued notifications", num);
                }
            }
            Err(error) => log::error!("Error sending queued notifications: {:?}", error),
        };
    }
}
