use chrono::{DateTime, NaiveDateTime, Utc};
use repository::{
    NotificationConfigKind, NotificationConfigRow, NotificationConfigRowRepository,
    NotificationType,
};
use service::{
    notification::enqueue::{create_notification_events, NotificationContext, NotificationTarget},
    service_provider::ServiceContext,
};

use crate::{parse::ColdChainPluginConfig, ColdChainError};

pub fn process_coldchain_alerts(
    ctx: &ServiceContext,
    current_time: NaiveDateTime,
) -> Result<usize, ColdChainError> {
    log::info!(
        "Processing cold_chain configurations due at {}",
        current_time
    );

    // Check if any cold chain configurations are due to be processed
    let configs = ctx
        .service_provider
        .notification_config_service
        .find_all_due_by_kind(ctx, NotificationConfigKind::ColdChain, current_time)
        .map_err(|e| ColdChainError::InternalError(format!("{:?}", e)))?;
    let num_configs = configs.len();

    println!("Found {} cold chain configurations to process", num_configs);

    Ok(num_configs)
}
