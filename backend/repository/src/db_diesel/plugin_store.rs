use diesel::prelude::*;

use super::{plugin_store::plugin_store::dsl as plugin_store_dsl, StorageConnection};
use crate::repository_error::RepositoryError;

table! {
    plugin_store (id) {
        id -> Text,
        plugin_name -> Text,
        key -> Text,
        value_string -> Text,
    }
}

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "plugin_store"]
pub struct PluginStoreRow {
    pub id: String,
    pub plugin_name: String,
    pub key: String,
    pub value_string: String,
}

pub struct PluginStoreRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> PluginStoreRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        PluginStoreRepository { connection }
    }

    pub fn upsert_one(&self, value: &PluginStoreRow) -> Result<(), RepositoryError> {
        diesel::replace_into(plugin_store_dsl::plugin_store)
            .values(value)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    fn get_row(
        &self,
        plugin_name: String,
        key: String,
    ) -> Result<Option<PluginStoreRow>, RepositoryError> {
        let result = plugin_store_dsl::plugin_store
            .filter(plugin_store_dsl::plugin_name.eq(plugin_name))
            .filter(plugin_store_dsl::key.eq(key))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn set_string(
        &self,
        plugin_name: String,
        key: String,
        value: String,
    ) -> Result<(), RepositoryError> {
        let id = format!("{}:{}", plugin_name, key);

        self.upsert_one(&PluginStoreRow {
            id: id,
            plugin_name: plugin_name,
            key: key,
            value_string: value,
        })
    }

    pub fn get_string(
        &self,
        plugin_name: String,
        key: String,
    ) -> Result<Option<String>, RepositoryError> {
        let row = self.get_row(plugin_name, key)?;
        Ok(row.and_then(|row| Some(row.value_string)))
    }
}
