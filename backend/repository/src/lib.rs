#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod database_settings;
pub mod db_diesel;
pub mod diesel_extensions;
pub mod diesel_macros;
pub mod mock;
mod repository_error;
pub mod test_db;

pub use self::db_diesel::*;
pub use self::repository_error::RepositoryError;
pub use database_settings::get_storage_connection_manager;
use diesel::sql_types::Text;
use diesel::{sql_query, RunQueryDsl};

mod tests;

embed_migrations!("./migrations/");

pub fn run_db_migrations(connection: &StorageConnection) -> Result<(), String> {
    embedded_migrations::run_with_output(&connection.connection, &mut std::io::stdout())
        .map_err(|err| format!("{}", err))
}

pub fn backup_sqlite(connection: &StorageConnection, path: &str) -> Result<(), String> {
    log::info!("Backing up sqlite database to {}", path);
    sql_query("VACUUM INTO ?")
        .bind::<Text, _>(path)
        .execute(&connection.connection)
        .map_err(|err| format!("{}", err))?;
    Ok(())
}

sql_function!(fn lower(x: Text) -> Text);
