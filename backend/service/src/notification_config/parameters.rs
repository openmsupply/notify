use std::collections::HashMap;

use crate::service_provider::ServiceContext;
use crate::notification::NotificationServiceError; // TODO: Fixup this import (or remove usage)
use super::query::NotificationConfig;
use repository::NotificationQueryRowRepository;

pub fn get_notification_parameters(
    ctx: &ServiceContext,
    notification_config: &NotificationConfig,
) -> Result<Vec<HashMap<String, serde_json::Value>>, NotificationServiceError> {
    // Fetch default parameters from config
    let params_string = match notification_config.parameters.len() {
        0 => "[{}]".to_string(),
        _ => notification_config.parameters.clone(),
    };
    let sql_params_string = match &notification_config.parameter_query_id {
        None => "[{}]".to_string(),
        Some(query_id) => get_sql_parameters(ctx, query_id)?,
    };

    // Parse both sets of parameter strings, then merge the resulting vectors
    let mut all_params: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&params_string)
        .map_err(|e| {
            NotificationServiceError::InternalError(format!(
                "Failed to parse notification config parameters (expecting an array of params_string): {:?} - {}",
                e, params_string
            ))
        })?;
    let mut sql_params: Vec<HashMap<String, serde_json::Value>>  = serde_json::from_str(&sql_params_string)
        .map_err(|e| {
            NotificationServiceError::InternalError(format!(
                "Failed to parse notification sql parameters (expecting an array of params_string): {:?} - {}",
                e, params_string
            ))
        })?;

    // TODO: Check if there's a cleaner version of this
    all_params.append(&mut sql_params);

    return Ok(all_params);
}

fn get_sql_parameters(
    ctx: &ServiceContext,
    parameter_query_id: &String,
) -> Result<String, NotificationServiceError> {
    // TODO: Maybe split these to a new database table
    let repository = NotificationQueryRowRepository::new(&ctx.connection);
    let query_record = repository.find_one_by_id(&parameter_query_id)?;

    let sql_query = match query_record {
        // TODO: Better error
        None => return Err(NotificationServiceError::InternalError("Temp error".to_string())),
        Some(record) => record.query,
    };

    // TODO: We could probably make parameter requests also accept parameters but that feels like
    // overkill

    let query_result = ctx
        .service_provider
        .datasource_service
        .run_sql_query(sql_query)
        .map_err(|_| {
            NotificationServiceError::InternalError("TODO: Better error".to_string())
        })?;

    // TODO: Idk how these results are formatted
    log::debug!("{}", query_result.results);
    return Ok(query_result.results);
}
