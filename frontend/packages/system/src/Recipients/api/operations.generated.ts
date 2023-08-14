import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type RecipientRowFragment = { __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode };

export type RecipientsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientSortInput> | Types.RecipientSortInput>;
}>;


export type RecipientsQuery = { __typename: 'FullQuery', recipients: { __typename: 'RecipientConnector', totalCount: number, nodes: Array<{ __typename: 'RecipientNode', id: string, name: string, toAddress: string, notificationType: Types.NotificationTypeNode }> } };

export type DeleteRecipientMutationVariables = Types.Exact<{
  recipientId: Types.Scalars['String']['input'];
}>;


export type DeleteRecipientMutation = { __typename: 'FullMutation', deleteRecipient: { __typename: 'DeleteResponse', id: string } };

export const RecipientRowFragmentDoc = gql`
    fragment RecipientRow on RecipientNode {
  __typename
  id
  name
  toAddress
  notificationType
}
    `;
export const RecipientsDocument = gql`
    query Recipients($filter: RecipientFilterInput, $page: PaginationInput, $sort: [RecipientSortInput!]) {
  recipients(filter: $filter, page: $page, sort: $sort) {
    __typename
    ... on RecipientConnector {
      __typename
      totalCount
      nodes {
        ...RecipientRow
      }
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
    deleteRecipient(variables: DeleteRecipientMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteRecipientMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteRecipientMutation>(DeleteRecipientDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteRecipient', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;