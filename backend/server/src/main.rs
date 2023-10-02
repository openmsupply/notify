#![allow(where_clauses_object_safety)]

use std::env;

use server::{configuration, start_server};
use service::settings::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        //Default rust log level to info
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let settings: Settings =
        configuration::get_configuration().expect("Failed to parse configuration settings");

    let off_switch = tokio::sync::mpsc::channel(1).1;
    start_server(settings, off_switch).await
}
