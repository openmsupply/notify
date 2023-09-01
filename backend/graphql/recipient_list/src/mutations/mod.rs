use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use graphql_types::types::IdResponse;
use service::recipient_list::ModifyRecipientListError;

mod add_member;
mod create;
mod delete;
mod remove_member;
mod update;

pub use add_member::*;
pub use create::*;
pub use delete::*;
pub use remove_member::*;
pub use update::*;

use crate::types::RecipientListNode;

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
