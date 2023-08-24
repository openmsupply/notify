import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type NotificationConfigRowFragment = { __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> };

export type NotificationConfigsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.NotificationConfigFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.NotificationConfigSortInput> | Types.NotificationConfigSortInput>;
}>;


export type NotificationConfigsQuery = { __typename: 'FullQuery', notificationConfigs: { __typename: 'NotificationConfigConnector', totalCount: number, nodes: Array<{ __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> }> } };

export type CreateNotificationConfigMutationVariables = Types.Exact<{
  input: Types.CreateNotificationConfigInput;
}>;


export type CreateNotificationConfigMutation = { __typename: 'FullMutation', createNotificationConfig: { __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string, auditLogs: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export const NotificationConfigRowFragmentDoc = gql`
    fragment NotificationConfigRow on NotificationConfigNode {
  id
  title
  kind
  configurationData
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
export const NotificationConfigsDocument = gql`
    query NotificationConfigs($filter: NotificationConfigFilterInput, $page: PaginationInput, $sort: [NotificationConfigSortInput!]) {
  notificationConfigs(filter: $filter, page: $page, sort: $sort) {
    ... on NotificationConfigConnector {
      totalCount
      nodes {
        ...NotificationConfigRow
      }
    }
  }
}
    ${NotificationConfigRowFragmentDoc}`;
export const CreateNotificationConfigDocument = gql`
    mutation createNotificationConfig($input: CreateNotificationConfigInput!) {
  createNotificationConfig(input: $input) {
    ... on NotificationConfigNode {
      ...NotificationConfigRow
    }
  }
}
    ${NotificationConfigRowFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    NotificationConfigs(variables?: NotificationConfigsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<NotificationConfigsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<NotificationConfigsQuery>(NotificationConfigsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'NotificationConfigs', 'query');
    },
    createNotificationConfig(variables: CreateNotificationConfigMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateNotificationConfigMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateNotificationConfigMutation>(CreateNotificationConfigDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createNotificationConfig', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;