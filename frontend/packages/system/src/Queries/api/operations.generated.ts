import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type NotificationQueryRowFragment = { __typename: 'NotificationQueryNode', id: string, name: string, description: string, query: string, requiredParameters: Array<string> };

export type NotificationQuerysQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.RecipientListFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.RecipientListSortInput> | Types.RecipientListSortInput>;
}>;


export type NotificationQuerysQuery = { __typename: 'FullQuery', notificationQuerys: { __typename: 'NotificationQueryConnector', totalCount: number, nodes: Array<{ __typename: 'NotificationQueryNode', id: string, name: string, description: string, query: string, requiredParameters: Array<string> }> } };

export type CreateNotificationQueryMutationVariables = Types.Exact<{
  input: Types.CreateNotificationQueryInput;
}>;


export type CreateNotificationQueryMutation = { __typename: 'FullMutation', createNotificationQuery: { __typename: 'NotificationQueryNode', id: string, name: string, description: string, query: string, requiredParameters: Array<string> } };

export type UpdateNotificationQueryMutationVariables = Types.Exact<{
  input: Types.UpdateNotificationQueryInput;
}>;


export type UpdateNotificationQueryMutation = { __typename: 'FullMutation', updateNotificationQuery: { __typename: 'NotificationQueryNode', id: string, name: string, description: string, query: string, requiredParameters: Array<string> } };

export type DeleteNotificationQueryMutationVariables = Types.Exact<{
  sqlRecipientListId: Types.Scalars['String']['input'];
}>;


export type DeleteNotificationQueryMutation = { __typename: 'FullMutation', deleteNotificationQuery: { __typename: 'DeleteResponse', id: string } };

export type TestNotificationQueryQueryVariables = Types.Exact<{
  sqlQuery?: Types.InputMaybe<Types.Scalars['String']['input']>;
  params?: Types.InputMaybe<Types.Scalars['String']['input']>;
}>;


export type TestNotificationQueryQuery = { __typename: 'FullQuery', runSqlQueryWithParameters: string };

export const NotificationQueryRowFragmentDoc = gql`
    fragment NotificationQueryRow on NotificationQueryNode {
  id
  name
  description
  query
  requiredParameters
}
    `;
export const NotificationQuerysDocument = gql`
    query notificationQuerys($filter: RecipientListFilterInput, $page: PaginationInput, $sort: [RecipientListSortInput!]) {
  notificationQuerys(filter: $filter, page: $page, sort: $sort) {
    ... on NotificationQueryConnector {
      totalCount
      nodes {
        ...NotificationQueryRow
      }
    }
  }
}
    ${NotificationQueryRowFragmentDoc}`;
export const CreateNotificationQueryDocument = gql`
    mutation createNotificationQuery($input: CreateNotificationQueryInput!) {
  createNotificationQuery(input: $input) {
    ... on NotificationQueryNode {
      ...NotificationQueryRow
    }
  }
}
    ${NotificationQueryRowFragmentDoc}`;
export const UpdateNotificationQueryDocument = gql`
    mutation updateNotificationQuery($input: UpdateNotificationQueryInput!) {
  updateNotificationQuery(input: $input) {
    ... on NotificationQueryNode {
      ...NotificationQueryRow
    }
  }
}
    ${NotificationQueryRowFragmentDoc}`;
export const DeleteNotificationQueryDocument = gql`
    mutation deleteNotificationQuery($sqlRecipientListId: String!) {
  deleteNotificationQuery(sqlRecipientListId: $sqlRecipientListId) {
    ... on DeleteResponse {
      id
    }
  }
}
    `;
export const TestNotificationQueryDocument = gql`
    query testNotificationQuery($sqlQuery: String, $params: String) {
  runSqlQueryWithParameters(sqlQuery: $sqlQuery, parameters: $params)
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    notificationQuerys(variables?: NotificationQuerysQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<NotificationQuerysQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<NotificationQuerysQuery>(NotificationQuerysDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'notificationQuerys', 'query');
    },
    createNotificationQuery(variables: CreateNotificationQueryMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateNotificationQueryMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateNotificationQueryMutation>(CreateNotificationQueryDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createNotificationQuery', 'mutation');
    },
    updateNotificationQuery(variables: UpdateNotificationQueryMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateNotificationQueryMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateNotificationQueryMutation>(UpdateNotificationQueryDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateNotificationQuery', 'mutation');
    },
    deleteNotificationQuery(variables: DeleteNotificationQueryMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteNotificationQueryMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteNotificationQueryMutation>(DeleteNotificationQueryDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteNotificationQuery', 'mutation');
    },
    testNotificationQuery(variables?: TestNotificationQueryQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<TestNotificationQueryQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<TestNotificationQueryQuery>(TestNotificationQueryDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'testNotificationQuery', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;