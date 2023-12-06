use repository::backup_sqlite;
use service::service_provider::ServiceContext;
use std::path::Path;
use tokio::time::sleep;

use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;

pub async fn auto_backup(service_context: ServiceContext) {
    if !service_context.service_provider.settings.backup.enabled {
        log::info!("Auto backup disabled");
        return;
    }

    let cron_string = service_context
        .service_provider
        .settings
        .backup
        .cron
        .clone();
    let schedule = match Schedule::from_str(&cron_string) {
        Ok(schedule) => schedule,
        Err(e) => {
            log::error!(
                "Error parsing backup cron string {}, defaulting to daily at midnight: {}",
                cron_string,
                e
            );
            Schedule::from_str("0 0 * * * *").unwrap()
        }
    };

    let file_path = Path::new(&service_context.service_provider.settings.backup.path)
        .join(&service_context.service_provider.settings.backup.filename);
    let file_path_name = file_path.to_string_lossy();

    let con = match service_context.service_provider.connection() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error connecting to database: {}", e);
            return;
        }
    };

    log::debug!("Backing up database to {}", file_path_name);
    for datetime in schedule.upcoming(Utc) {
        println!("Next backup at  {}", datetime);
        let now_utc = Utc::now();
        let duration = datetime.signed_duration_since(now_utc);
        sleep(duration.to_std().unwrap_or_default()).await;

        // delete the old backup file
        if file_path.exists() {
            match std::fs::remove_file(&file_path) {
                Ok(()) => log::info!("Deleted old backup file"),
                Err(e) => log::error!("Error deleting old backup file: {}", e),
            }
        }

        match backup_sqlite(&con, &file_path_name) {
            Ok(()) => log::info!("Backup successful"),
            Err(e) => log::error!("Backup failed: {}", e),
        }
    }
}
