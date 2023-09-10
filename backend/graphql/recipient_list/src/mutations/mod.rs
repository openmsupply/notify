use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use graphql_types::types::IdResponse;
use service::{
    recipient_list::ModifyRecipientListError, sql_recipient_list::ModifySqlRecipientListError,
};

mod add_member;
mod create;
mod create_sql;
mod delete;
mod delete_sql;
mod remove_member;
mod update;
mod update_sql;

pub use add_member::*;
pub use create::*;
pub use create_sql::*;
pub use delete::*;
pub use delete_sql::*;
pub use remove_member::*;
pub use update::*;
pub use update_sql::*;

use crate::types::{RecipientListNode, SqlRecipientListNode};

#[derive(Union)]
pub enum ModifyRecipientListResponse {
    Response(RecipientListNode),
}

#[derive(Union)]
pub enum ModifyRecipientListMembersResponse {
    Response(IdResponse),
}

pub fn map_error<T>(error: ModifyRecipientListError) -> Result<T> {
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyRecipientListError::RecipientListDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientListError::InvalidRecipientListName => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListMemberAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListMemberDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyRecipientListError::DatabaseError(_) => InternalError(formatted_error),
        ModifyRecipientListError::GenericError(s) => InternalError(s),
    };

    Err(graphql_error.extend())
}

#[derive(Union)]
pub enum ModifySqlRecipientListResponse {
    Response(SqlRecipientListNode),
}

pub fn map_sql_error<T>(error: ModifySqlRecipientListError) -> Result<T> {
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifySqlRecipientListError::SqlRecipientListDoesNotExist => BadUserInput(formatted_error),
        ModifySqlRecipientListError::SqlRecipientListAlreadyExists => BadUserInput(formatted_error),
        ModifySqlRecipientListError::InvalidSqlRecipientListName => BadUserInput(formatted_error),
        ModifySqlRecipientListError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifySqlRecipientListError::DatabaseError(_) => InternalError(formatted_error),
        ModifySqlRecipientListError::InternalError(s) => InternalError(s),
        ModifySqlRecipientListError::BadUserInput(s) => BadUserInput(s),
    };

    Err(graphql_error.extend())
}
