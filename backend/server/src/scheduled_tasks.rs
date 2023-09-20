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
        // Note: If a plugin starts an infinite loop here, we're a bit stuffed.
        // Hopefully people will be smart enough not to do that?
        // TODO: would be nice to run plugins in parallel and have a timeout of some sort?
        // It probably should also be done in spawn_blocking thread so that it doesn't block the rt thread.
        for plugin in &plugins {
            let result = plugin.tick(&service_context);
            match result {
                Ok(_) => {}
                Err(error) => {
                    log::error!("Error processing {} plugin: {:?} ", plugin.name(), error)
                }
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
