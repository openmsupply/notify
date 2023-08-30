import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type BasicRecipientRowFragment = { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode };

export type RecipientRowFragment = { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> };

export type RecipientsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientSortInput> | Types.RecipientSortInput>;
}>;


export type RecipientsQuery = { __typename: 'FullQuery', recipients: { __typename: 'RecipientConnector', totalCount: number, nodes: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }> } };

export type BasicRecipientsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientSortInput> | Types.RecipientSortInput>;
}>;


export type BasicRecipientsQuery = { __typename: 'FullQuery', recipients: { __typename: 'RecipientConnector', totalCount: number, nodes: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode }> } };

export type CreateRecipientMutationVariables = Types.Exact<{
  input: Types.CreateRecipientInput;
}>;


export type CreateRecipientMutation = { __typename: 'FullMutation', createRecipient: { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export type UpdateRecipientMutationVariables = Types.Exact<{
  input: Types.UpdateRecipientInput;
}>;


export type UpdateRecipientMutation = { __typename: 'FullMutation', updateRecipient: { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export type DeleteRecipientMutationVariables = Types.Exact<{
  recipientId: Types.Scalars['String']['input'];
}>;


export type DeleteRecipientMutation = { __typename: 'FullMutation', deleteRecipient: { __typename: 'DeleteResponse', id: string } };

export type RecipientListRowFragment = { __typename: 'RecipientListNode', id: string, name: string, description: string, sqlQuery?: string | null, recipients: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }>, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> };

export type RecipientListsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientListFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientListSortInput> | Types.RecipientListSortInput>;
}>;


export type RecipientListsQuery = { __typename: 'FullQuery', recipientLists: { __typename: 'RecipientListConnector', totalCount: number, nodes: Array<{ __typename: 'RecipientListNode', id: string, name: string, description: string, sqlQuery?: string | null, recipients: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }>, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }> } };

export type CreateRecipientListMutationVariables = Types.Exact<{
  input: Types.CreateRecipientListInput;
}>;


export type CreateRecipientListMutation = { __typename: 'FullMutation', createRecipientList: { __typename: 'RecipientListNode', id: string, name: string, description: string, sqlQuery?: string | null, recipients: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }>, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export type UpdateRecipientListMutationVariables = Types.Exact<{
  input: Types.UpdateRecipientListInput;
}>;


export type UpdateRecipientListMutation = { __typename: 'FullMutation', updateRecipientList: { __typename: 'RecipientListNode', id: string, name: string, description: string, sqlQuery?: string | null, recipients: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }>, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export type AddRecipientToListMutationVariables = Types.Exact<{
  input: Types.AddRecipientToListInput;
}>;


export type AddRecipientToListMutation = { __typename: 'FullMutation', addRecipientToList: { __typename: 'IdResponse', id: string } };

export type RemoveRecipientFromListMutationVariables = Types.Exact<{
  input: Types.RemoveRecipientFromListInput;
}>;


export type RemoveRecipientFromListMutation = { __typename: 'FullMutation', removeRecipientFromList: { __typename: 'IdResponse', id: string } };

export type DeleteRecipientListMutationVariables = Types.Exact<{
  recipientListId: Types.Scalars['String']['input'];
}>;


export type DeleteRecipientListMutation = { __typename: 'FullMutation', deleteRecipientList: { __typename: 'DeleteResponse', id: string } };

export type SendTestTelegramMessageMutationVariables = Types.Exact<{
  chatId: Types.Scalars['String']['input'];
}>;


export type SendTestTelegramMessageMutation = { __typename: 'FullMutation', sendTestTelegramMessage: { __typename: 'TelegramMessageNode', chatName: string, message: string } };

export type RecipientsViaSqlQueryVariables = Types.Exact<{
  sqlQuery?: Types.InputMaybe<Types.Scalars['String']['input']>;
}>;


export type RecipientsViaSqlQuery = { __typename: 'FullQuery', runSqlQuery: string };

export const BasicRecipientRowFragmentDoc = gql`
    fragment BasicRecipientRow on RecipientNode {
  id
  name
  toAddress
  notificationType
}
    `;
export const RecipientRowFragmentDoc = gql`
    fragment RecipientRow on RecipientNode {
  id
  name
  toAddress
  notificationType
  auditLogs {
    datetime
    id
    recordId
    recordType
    user {
      username
    }
  }
}
    `;
export const RecipientListRowFragmentDoc = gql`
    fragment RecipientListRow on RecipientListNode {
  id
  name
  description
  recipients {
    ...RecipientRow
  }
  auditLogs {
    datetime
    id
    recordId
    recordType
    user {
      username
    }
  }
  sqlQuery
}
    ${RecipientRowFragmentDoc}`;
export const RecipientsDocument = gql`
    query Recipients($filter: RecipientFilterInput, $page: PaginationInput, $sort: [RecipientSortInput!]) {
  recipients(filter: $filter, page: $page, sort: $sort) {
    ... on RecipientConnector {
      totalCount
      nodes {
        ...RecipientRow
      }
    }
  }
}
    ${RecipientRowFragmentDoc}`;
export const BasicRecipientsDocument = gql`
    query BasicRecipients($filter: RecipientFilterInput, $page: PaginationInput, $sort: [RecipientSortInput!]) {
  recipients(filter: $filter, page: $page, sort: $sort) {
    ... on RecipientConnector {
      totalCount
      nodes {
        ...BasicRecipientRow
      }
    }
  }
}
    ${BasicRecipientRowFragmentDoc}`;
export const CreateRecipientDocument = gql`
    mutation createRecipient($input: CreateRecipientInput!) {
  createRecipient(input: $input) {
    ... on RecipientNode {
      ...RecipientRow
    }
  }
}
    ${RecipientRowFragmentDoc}`;
export const UpdateRecipientDocument = gql`
    mutation updateRecipient($input: UpdateRecipientInput!) {
  updateRecipient(input: $input) {
    ... on RecipientNode {
      ...RecipientRow
    }
  }
}
    ${RecipientRowFragmentDoc}`;
export const DeleteRecipientDocument = gql`
    mutation deleteRecipient($recipientId: String!) {
  deleteRecipient(recipientId: $recipientId) {
    ... on DeleteResponse {
      id
    }
  }
}
    `;
export const RecipientListsDocument = gql`
    query RecipientLists($filter: RecipientListFilterInput, $page: PaginationInput, $sort: [RecipientListSortInput!]) {
  recipientLists(filter: $filter, page: $page, sort: $sort) {
    ... on RecipientListConnector {
      totalCount
      nodes {
        ...RecipientListRow
      }
    }
  }
}
    ${RecipientListRowFragmentDoc}`;
export const CreateRecipientListDocument = gql`
    mutation createRecipientList($input: CreateRecipientListInput!) {
  createRecipientList(input: $input) {
    ... on RecipientListNode {
      ...RecipientListRow
    }
  }
}
    ${RecipientListRowFragmentDoc}`;
export const UpdateRecipientListDocument = gql`
    mutation updateRecipientList($input: UpdateRecipientListInput!) {
  updateRecipientList(input: $input) {
    ... on RecipientListNode {
      ...RecipientListRow
    }
  }
}
    ${RecipientListRowFragmentDoc}`;
export const AddRecipientToListDocument = gql`
    mutation addRecipientToList($input: AddRecipientToListInput!) {
  addRecipientToList(input: $input) {
    ... on IdResponse {
      id
    }
  }
}
    `;
export const RemoveRecipientFromListDocument = gql`
    mutation removeRecipientFromList($input: RemoveRecipientFromListInput!) {
  removeRecipientFromList(input: $input) {
    ... on IdResponse {
      id
    }
  }
}
    `;
export const DeleteRecipientListDocument = gql`
    mutation deleteRecipientList($recipientListId: String!) {
  deleteRecipientList(recipientListId: $recipientListId) {
    ... on DeleteResponse {
      id
    }
  }
}
    `;
export const SendTestTelegramMessageDocument = gql`
    mutation sendTestTelegramMessage($chatId: String!) {
  sendTestTelegramMessage(chatId: $chatId) {
    ... on TelegramMessageNode {
      __typename
      chatName
      message
    }
  }
}
    `;
export const RecipientsViaSqlDocument = gql`
    query recipientsViaSQL($sqlQuery: String) {
  runSqlQuery(sqlQuery: $sqlQuery)
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    Recipients(variables?: RecipientsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RecipientsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<RecipientsQuery>(RecipientsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'Recipients', 'query');
    },
    BasicRecipients(variables?: BasicRecipientsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<BasicRecipientsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<BasicRecipientsQuery>(BasicRecipientsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'BasicRecipients', 'query');
    },
    createRecipient(variables: CreateRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateRecipientMutation>(CreateRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createRecipient', 'mutation');
    },
    updateRecipient(variables: UpdateRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateRecipientMutation>(UpdateRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateRecipient', 'mutation');
    },
    deleteRecipient(variables: DeleteRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteRecipientMutation>(DeleteRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteRecipient', 'mutation');
    },
    RecipientLists(variables?: RecipientListsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RecipientListsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<RecipientListsQuery>(RecipientListsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'RecipientLists', 'query');
    },
    createRecipientList(variables: CreateRecipientListMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateRecipientListMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateRecipientListMutation>(CreateRecipientListDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createRecipientList', 'mutation');
    },
    updateRecipientList(variables: UpdateRecipientListMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateRecipientListMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateRecipientListMutation>(UpdateRecipientListDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateRecipientList', 'mutation');
    },
    addRecipientToList(variables: AddRecipientToListMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<AddRecipientToListMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<AddRecipientToListMutation>(AddRecipientToListDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'addRecipientToList', 'mutation');
    },
    removeRecipientFromList(variables: RemoveRecipientFromListMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RemoveRecipientFromListMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<RemoveRecipientFromListMutation>(RemoveRecipientFromListDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'removeRecipientFromList', 'mutation');
    },
    deleteRecipientList(variables: DeleteRecipientListMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteRecipientListMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteRecipientListMutation>(DeleteRecipientListDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteRecipientList', 'mutation');
    },
    sendTestTelegramMessage(variables: SendTestTelegramMessageMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<SendTestTelegramMessageMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<SendTestTelegramMessageMutation>(SendTestTelegramMessageDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'sendTestTelegramMessage', 'mutation');
    },
    recipientsViaSQL(variables?: RecipientsViaSqlQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RecipientsViaSqlQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<RecipientsViaSqlQuery>(RecipientsViaSqlDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'recipientsViaSQL', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;