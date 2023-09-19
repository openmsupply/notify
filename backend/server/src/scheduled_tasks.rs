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

        // Process plugins
        for plugin in &plugins {
            let result = plugin.tick();
            match result {
                Ok(_) => {}
                Err(error) => log::error!("Error processing plugin: {:?}", error),
            }
        }
    }
}
