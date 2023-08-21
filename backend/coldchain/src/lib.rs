use chrono::NaiveDateTime;
use serde::Serialize;

/*

Temperature Alerts will look something like this...
-----------------------
High temperature alert!

Facility: Store A
Location: Fridge 1
Sensor: E5:4G:D4:6D:A4

Date: 17 Jul 2023
Time: 17:04

Temperature: 10° C
-----------------------
*/

#[derive(Debug, Serialize)]
pub struct TemperatureAlert {
    pub store_id: String,
    pub store_name: String,
    pub location_id: String,
    pub location_name: String,
    pub sensor_id: String,
    pub sensor_name: String,
    pub datetime: NaiveDateTime,
    pub temperature: f64,
}

// pub async fn send_high_temperature_alert_telegram(
//     alert: TemperatureAlert,
//     telegram_client: TelegramClient,
//     telegram_chat_id: String,
// ) -> () {
//     let result = render_template("coldchain/telegram/temperature.html", &alert);
//     match result {
//         Ok(html) => {
//             let result = telegram_client
//                 .send_html_message(&telegram_chat_id, &html)
//                 .await;
//             match result {
//                 Ok(_) => {
//                     println!("Successfully sent high temperature alert telegram");
//                 }
//                 Err(e) => {
//                     panic!("Error sending high temperature alert telegram: {:?}", e);
//                     todo!();
//                 }
//             }
//         }
//         Err(e) => {
//             panic!("Error rendering temperature alert template: {:?}", e);
//             todo!();
//         }
//     }
// }

#[cfg(test)]
// #[cfg(feature = "telegram-tests")]
mod tests {
    // use std::str::FromStr;

    // use service::service_provider::ServiceProvider;

    // use super::*;

    // use std::sync::Arc;

    // #[tokio::test]
    // async fn test_send_high_temperature_alert_telegram() {
    //     let client = TelegramClient::new(get_telegram_token_from_env());

    //     let example_alert = TemperatureAlert {
    //         store_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b1".to_string(),
    //         store_name: "Store A".to_string(),
    //         location_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b2".to_string(),
    //         location_name: "Fridge 1".to_string(),
    //         sensor_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b3".to_string(),
    //         sensor_name: "E5:4G:D4:6D:A4".to_string(),
    //         datetime: NaiveDateTime::from_str("2023-07-17T17:04:00").unwrap(),
    //         temperature: 10.12345,
    //     };

    //     let result = send_high_temperature_alert_telegram(
    //         example_alert,
    //         client,
    //         get_telegram_chat_id_from_env(),
    //     )
    //     .await;

    //     assert_eq!(result, ());
    // }
}
