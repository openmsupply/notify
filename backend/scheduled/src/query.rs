use std::collections::HashMap;

use log::info;
use repository::{EqualFilter, NotificationQueryFilter, NotificationQueryRepository};
use serde_json::json;
use service::service_provider::ServiceContext;

use crate::{parse::ScheduledNotificationPluginConfig, NotificationError};

pub fn get_notification_query_results(
    ctx: &ServiceContext,
    parameters: serde_json::Value,
    config: &ScheduledNotificationPluginConfig,
) -> Result<HashMap<String, serde_json::Value>, NotificationError> {
    let mut query_results = HashMap::new();

    // get all the configured queries

    let notification_config_repo = NotificationQueryRepository::new(&ctx.connection);

    let queries = notification_config_repo
        .query_by_filter(NotificationQueryFilter::new().id(EqualFilter::equal_any(
            config.notification_query_ids.clone(),
        )))
        .map_err(|e| {
            NotificationError::InternalError(format!("Unable to get notification queries: {:?}", e))
        })?;

    // loop through all the notification query ids, run them, and store the results
    for query in queries {
        let now = chrono::Utc::now();
        let result = ctx
            .service_provider
            .datasource_service
            .run_sql_query_with_parameters(query.query.clone(), parameters.clone());
        let query_json = match result {
            Ok(result) => serde_json::from_str(&result.results)
                .unwrap_or_else(|_| json!([{"error": "Unable to parse query result"}])),
            Err(e) => {
                log::error!(
                    "Error running query {} for {}({}) : {:?}",
                    query.reference_name,
                    config.title,
                    config.id,
                    e
                );
                json!([{"error": "error running query", "query": query.query, "parameters": parameters}])
            }
        };
        let end_time = chrono::Utc::now();
        info!(
            "Query {} took {}ms",
            query.reference_name,
            (end_time - now).num_milliseconds()
        );

        query_results.insert(query.reference_name, query_json);
    }

    Ok(query_results)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use repository::{
        mock::{
            mock_notification_query_with_no_param_2_rows, mock_notification_query_with_params,
            MockDataInserts,
        },
        test_db::setup_all,
        NotificationConfigStatus,
    };
    use util::uuid::uuid;

    use service::{
        notification_config::query::NotificationConfig, service_provider::ServiceProvider,
        test_utils::get_test_settings,
    };

    use super::*;

    // Test that we get the correct results when we have a notification query with no parameters
    #[tokio::test]
    async fn test_get_notification_query_results_no_param() {
        let (_, _, connection_manager, _) = setup_all(
            "test_get_notification_query_results_no_param",
            MockDataInserts::none().notification_queries(),
        )
        .await;
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // There are two mock notifications, one with a parameters and one without

        // 1. Check we get the result with 1 query and no parameters

        // Create an empty notification config with no params to use

        let all_params: Vec<serde_json::Value> = serde_json::from_str("[{}]")
            .map_err(|e| {
                NotificationError::InternalError(format!(
                    "Failed to parse notification parameters: {:?}",
                    e
                ))
            })
            .unwrap();

        let config = ScheduledNotificationPluginConfig {
            notification_query_ids: vec![mock_notification_query_with_no_param_2_rows().id],
            ..Default::default()
        };

        // Call the function being tested
        let query_results =
            get_notification_query_results(&context, all_params[0].clone(), &config).unwrap();

        let result_key = mock_notification_query_with_no_param_2_rows().reference_name;

        // Check we got the result we expected
        assert_eq!(
            query_results.get(&result_key).unwrap(),
            &json!([
                {
                    "latest_temperature": 1.25,
                    "sensor_name": "sensor1",
                    "sensor_limit": -10
                },
                {
                    "latest_temperature": 1.51,
                    "sensor_name": "sensor2",
                    "sensor_limit": -10
                }
            ])
        );
    }

    // Test that we get the correct results when we have a notification query with parameters
    #[tokio::test]
    async fn test_get_notification_query_results_params() {
        let (_, _, connection_manager, _) = setup_all(
            "test_get_notification_query_results_params",
            MockDataInserts::none().notification_queries(),
        )
        .await;
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // There are two mock notifications, one with a parameters and one without

        // 2. Check we get the result with 1 query and parameters (sensor_limit and latest_temperature)

        // Create a notification config with the correct params
        let notification_config = NotificationConfig {
            id: uuid(),
            parameters: "[{\"sensor_limit\": \"8\", \"latest_temperature\": \"8.5\"}]".to_string(),
            status: NotificationConfigStatus::Enabled,
            ..Default::default()
        };

        let all_params: Vec<serde_json::Value> =
            serde_json::from_str(&notification_config.parameters)
                .map_err(|e| {
                    NotificationError::InternalError(format!(
                        "Failed to parse notification parameters: {:?}",
                        e
                    ))
                })
                .unwrap();

        let config = ScheduledNotificationPluginConfig {
            notification_query_ids: vec![mock_notification_query_with_params().id],
            ..Default::default()
        };

        // Call the function being tested
        let query_results =
            get_notification_query_results(&context, all_params[0].clone(), &config).unwrap();

        let result_key = mock_notification_query_with_params().reference_name;

        // Check we got the result we expected
        assert_eq!(
            query_results.get(&result_key).unwrap(),
            &json!([
                {
                    "latest_temperature": 8.5,
                    "sensor_limit": 8,
                    "is_above_limit": true,
                }
            ])
        );
    }

    // Test that we get the correct results when we have a 2 notification queries, one with parameters and one without
    #[tokio::test]
    async fn test_get_notification_query_results_2_queries() {
        let (_, _, connection_manager, _) = setup_all(
            "test_get_notification_query_results_2_queries",
            MockDataInserts::none().notification_queries(),
        )
        .await;
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // There are two mock notifications, one with a parameters and one without

        // 3. Check we get the results with 2 queries and parameters (sensor_limit and latest_temperature)

        // Create a notification config
        let notification_config = NotificationConfig {
            id: uuid(),
            parameters: "[{\"sensor_limit\": \"8\", \"latest_temperature\": \"8.5\"}]".to_string(),
            status: NotificationConfigStatus::Enabled,
            ..Default::default()
        };

        let all_params: Vec<serde_json::Value> =
            serde_json::from_str(&notification_config.parameters)
                .map_err(|e| {
                    NotificationError::InternalError(format!(
                        "Failed to parse notification parameters: {:?}",
                        e
                    ))
                })
                .unwrap();

        let config = ScheduledNotificationPluginConfig {
            notification_query_ids: vec![
                mock_notification_query_with_params().id,
                mock_notification_query_with_no_param_2_rows().id,
            ],
            ..Default::default()
        };

        // Call the function being tested
        let query_results =
            get_notification_query_results(&context, all_params[0].clone(), &config).unwrap();

        // Check we got the 2 results we expected
        assert_eq!(query_results.len(), 2);

        let result_key = mock_notification_query_with_params().reference_name;

        // Check we got the result we expected for the first query
        assert_eq!(
            query_results.get(&result_key).unwrap(),
            &json!([
                {
                    "latest_temperature": 8.5,
                    "sensor_limit": 8,
                    "is_above_limit": true,
                }
            ])
        );

        let result_key = mock_notification_query_with_no_param_2_rows().reference_name;

        // Check we got the result we expected for the second query
        assert_eq!(
            query_results.get(&result_key).unwrap(),
            &json!([
                {
                    "latest_temperature": 1.25,
                    "sensor_name": "sensor1",
                    "sensor_limit": -10
                },
                {
                    "latest_temperature": 1.51,
                    "sensor_name": "sensor2",
                    "sensor_limit": -10
                }
            ])
        );
    }

    // Test we get an error back if we try to run a query missing it's parameters
    #[tokio::test]
    async fn test_get_notification_query_results_missing_params() {
        let (_, _, connection_manager, _) = setup_all(
            "test_get_notification_query_results_missing_params",
            MockDataInserts::none().notification_queries(),
        )
        .await;
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // There are two mock notifications, one with a parameters and one without

        // 4. Check we get an error if we try to run a query missing it's parameters

        // Create a notification config with no params to use
        let notification_config = NotificationConfig {
            id: uuid(),
            parameters: "[{}]".to_string(),
            status: NotificationConfigStatus::Enabled,
            ..Default::default()
        };

        let all_params: Vec<serde_json::Value> =
            serde_json::from_str(&notification_config.parameters)
                .map_err(|e| {
                    NotificationError::InternalError(format!(
                        "Failed to parse notification parameters: {:?}",
                        e
                    ))
                })
                .unwrap();

        let config = ScheduledNotificationPluginConfig {
            notification_query_ids: vec![mock_notification_query_with_params().id],
            ..Default::default()
        };

        // Call the function being tested
        let result =
            get_notification_query_results(&context, all_params[0].clone(), &config).unwrap();

        // Check we got the error we expected
        assert_eq!(
            result.get("query1").unwrap()[0]["error"].as_str().unwrap(),
            "error running query"
        );
    }
}
