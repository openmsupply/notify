use super::validate::check_sql_recipient_list_exists;
use crate::service_provider::ServiceContext;
use repository::{
    RepositoryError, SqlRecipientListRow, SqlRecipientListRowRepository, StorageConnection,
};

#[derive(PartialEq, Debug)]
pub enum DeleteSqlRecipientListError {
    SqlRecipientListDoesNotExist,
    DatabaseError(RepositoryError),
}

pub fn delete_sql_recipient_list(
    ctx: &ServiceContext,
    sql_recipient_list_id: &str,
) -> Result<String, DeleteSqlRecipientListError> {
    let sql_recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            let sql_recipient_list_row = validate(connection, sql_recipient_list_id)?;

            let sql_recipient_list_repo = SqlRecipientListRowRepository::new(connection);
            match sql_recipient_list_repo.delete(sql_recipient_list_id) {
                Ok(_) => {}
                Err(err) => {
                    return Err(DeleteSqlRecipientListError::from(err));
                }
            };

            Ok(sql_recipient_list_row)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(sql_recipient_list.id)
}

pub fn validate(
    connection: &StorageConnection,
    sql_recipient_list_id: &str,
) -> Result<SqlRecipientListRow, DeleteSqlRecipientListError> {
    let sql_recipient_list_row =
        match check_sql_recipient_list_exists(sql_recipient_list_id, connection)? {
            Some(sql_recipient_list_row) => sql_recipient_list_row,
            None => return Err(DeleteSqlRecipientListError::SqlRecipientListDoesNotExist),
        };

    Ok(sql_recipient_list_row)
}

impl From<RepositoryError> for DeleteSqlRecipientListError {
    fn from(error: RepositoryError) -> Self {
        DeleteSqlRecipientListError::DatabaseError(error)
    }
}
