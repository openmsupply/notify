import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type RecipientRowFragment = { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> };

export type RecipientsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientSortInput> | Types.RecipientSortInput>;
}>;


export type RecipientsQuery = { __typename: 'FullQuery', recipients: { __typename: 'RecipientConnector', totalCount: number, nodes: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }> } };

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

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    Recipients(variables?: RecipientsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RecipientsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<RecipientsQuery>(RecipientsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'Recipients', 'query');
    },
    createRecipient(variables: CreateRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateRecipientMutation>(CreateRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createRecipient', 'mutation');
    },
    updateRecipient(variables: UpdateRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateRecipientMutation>(UpdateRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateRecipient', 'mutation');
    },
    deleteRecipient(variables: DeleteRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteRecipientMutation>(DeleteRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteRecipient', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;