import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type NotificationEventRowFragment = { __typename: 'NotificationEventNode', id: string, title: string, sentAt?: string | null, message: string, errorMessage?: string | null, createdAt: string, status: Types.EventStatus, toAddress: string, updatedAt: string, notificationType: Types.NotificationTypeNode, notificationConfigId?: string | null, notificationConfig?: { __typename: 'NotificationConfigNode', title: string, kind: Types.ConfigKind } | null };

export type NotificationEventsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.NotificationEventFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.NotificationEventSortInput> | Types.NotificationEventSortInput>;
}>;


export type NotificationEventsQuery = { __typename: 'FullQuery', notificationEvents: { __typename: 'NotificationEventConnector', totalCount: number, nodes: Array<{ __typename: 'NotificationEventNode', id: string, title: string, sentAt?: string | null, message: string, errorMessage?: string | null, createdAt: string, status: Types.EventStatus, toAddress: string, updatedAt: string, notificationType: Types.NotificationTypeNode, notificationConfigId?: string | null, notificationConfig?: { __typename: 'NotificationConfigNode', title: string, kind: Types.ConfigKind } | null }> } };

export const NotificationEventRowFragmentDoc = gql`
    fragment NotificationEventRow on NotificationEventNode {
  id
  title
  sentAt
  message
  errorMessage
  createdAt
  status
  toAddress
  updatedAt
  notificationType
  notificationConfigId
  notificationConfig {
    title
    kind
  }
}
    `;
export const NotificationEventsDocument = gql`
    query NotificationEvents($filter: NotificationEventFilterInput, $page: PaginationInput, $sort: [NotificationEventSortInput!]) {
  notificationEvents(filter: $filter, page: $page, sort: $sort) {
    ... on NotificationEventConnector {
      totalCount
      nodes {
        ...NotificationEventRow
      }
    }
  }
}
    ${NotificationEventRowFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    NotificationEvents(variables?: NotificationEventsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<NotificationEventsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<NotificationEventsQuery>(NotificationEventsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'NotificationEvents', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;