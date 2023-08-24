import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type NotificationConfigRowFragment = { __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string };

export type NotificationConfigsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.NotificationConfigFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.NotificationConfigSortInput> | Types.NotificationConfigSortInput>;
}>;


export type NotificationConfigsQuery = { __typename: 'FullQuery', notificationConfigs: { __typename: 'NotificationConfigConnector', totalCount: number, nodes: Array<{ __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string }> } };

export type CreateNotificationConfigMutationVariables = Types.Exact<{
  input: Types.CreateNotificationConfigInput;
}>;


export type CreateNotificationConfigMutation = { __typename: 'FullMutation', createNotificationConfig: { __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string } };

export type UpdateNotificationConfigMutationVariables = Types.Exact<{
  input: Types.UpdateNotificationConfigInput;
}>;


export type UpdateNotificationConfigMutation = { __typename: 'FullMutation', updateNotificationConfig: { __typename: 'NotificationConfigNode', id: string, title: string, kind: Types.ConfigKind, configurationData: string } };

export type DeleteNotificationConfigMutationVariables = Types.Exact<{
  id: Types.Scalars['String']['input'];
}>;


export type DeleteNotificationConfigMutation = { __typename: 'FullMutation', deleteNotificationConfig: { __typename: 'DeleteResponse', id: string } };

export const NotificationConfigRowFragmentDoc = gql`
    fragment NotificationConfigRow on NotificationConfigNode {
  id
  title
  kind
  configurationData
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
export const UpdateNotificationConfigDocument = gql`
    mutation updateNotificationConfig($input: UpdateNotificationConfigInput!) {
  updateNotificationConfig(input: $input) {
    ... on NotificationConfigNode {
      ...NotificationConfigRow
    }
  }
}
    ${NotificationConfigRowFragmentDoc}`;
export const DeleteNotificationConfigDocument = gql`
    mutation deleteNotificationConfig($id: String!) {
  deleteNotificationConfig(id: $id) {
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
    NotificationConfigs(variables?: NotificationConfigsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<NotificationConfigsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<NotificationConfigsQuery>(NotificationConfigsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'NotificationConfigs', 'query');
    },
    createNotificationConfig(variables: CreateNotificationConfigMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateNotificationConfigMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateNotificationConfigMutation>(CreateNotificationConfigDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'createNotificationConfig', 'mutation');
    },
    updateNotificationConfig(variables: UpdateNotificationConfigMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateNotificationConfigMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateNotificationConfigMutation>(UpdateNotificationConfigDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateNotificationConfig', 'mutation');
    },
    deleteNotificationConfig(variables: DeleteNotificationConfigMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<DeleteNotificationConfigMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<DeleteNotificationConfigMutation>(DeleteNotificationConfigDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'deleteNotificationConfig', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;